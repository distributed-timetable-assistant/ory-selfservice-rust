# Ory Shield UI

A self-hosted Rust authentication application that provides the browser-facing UI for **Ory Kratos Self-Service** and the **Login & Consent application required by Ory Hydra**.

The goal is deliberately narrow: deploy one small service next to Kratos and Hydra so DiTA and other OIDC clients can redirect users to a complete, independent authentication service instead of embedding username/password screens inside each application.

## Scope

Ory Shield UI is an application, not a replacement for Kratos or Hydra.

It owns the browser-facing experience and orchestration around them:

- Kratos Self-Service UI: login, registration, recovery, verification, settings, password changes, MFA/TOTP, and other enabled Kratos methods.
- Hydra Login application: consumes `login_challenge`, authenticates the user through Kratos, and accepts or rejects the Hydra login request.
- Hydra Consent application: consumes `consent_challenge`, presents the requested scopes/claims, and accepts or rejects the consent request.
- Redirect handling: returns the browser to the URL selected by Kratos or Hydra after a successful flow.

It does **not** become an identity database, OAuth authorization server, or password store. Kratos remains the identity/authentication authority; Hydra remains the OAuth 2.0 / OpenID Connect authorization server.

## Non-Goals

Shield does not:

- issue OAuth2/OIDC tokens;
- store passwords or identities;
- expose the Hydra Admin API to browsers;
- implement an independent session database;
- replace Kratos Self-Service flows;
- require every DiTA portal to implement its own login UI.

## Architecture

```text
                         Browser
                            |
                            | OIDC authorization request
                            v
                     +--------------+
                     |     DiTA     |
                     | OIDC Client  |
                     +------+-------+
                            |
                            | /oauth2/auth
                            v
                     +--------------+
                     |    Hydra     |
                     | OAuth2/OIDC  |
                     +------+-------+
                            |
               +------------+------------+
               |                         |
        login_challenge          consent_challenge
               |                         |
               +------------+------------+
                            |
                            v
                    +---------------+
                    | Ory Shield UI |
                    |               |
                    | Axum BFF      |
                    | Leptos UI     |
                    +-------+-------+
                            |
                 +----------+----------+
                 |                     |
                 | Kratos Public API   | Hydra Admin API
                 v                     v
          +-------------+       +-------------+
          |   Kratos    |       |    Hydra    |
          | Identity +  |       | OAuth2/OIDC |
          | Self-Service|       |   Admin     |
          +-------------+       +-------------+
```

### Responsibility boundaries

**Kratos** is the source of truth for identity and authentication state. Shield starts and updates Kratos browser/self-service flows and renders their `ui.nodes`.

**Hydra** is the source of truth for OAuth2/OIDC authorization. Shield resolves Hydra login and consent challenges through the Hydra Admin API.

**Shield** is the browser-facing orchestration layer between the two systems. It must never persist credentials or duplicate Kratos identity state.

## Core Flows

### 1. Direct Login

```text
GET /login
  -> Kratos creates/loads login flow
  -> Shield renders Kratos UI nodes
  -> Browser submits form
  -> Shield updates Kratos flow
  -> Kratos creates session
  -> Shield redirects to the configured return URL
```

### 2. Registration

```text
GET /registration
  -> Kratos registration flow
  -> render ui.nodes
  -> submit to Kratos
  -> optional verification / hooks handled by Kratos
  -> redirect according to Kratos flow result
```

Kratos currently supports richer registration behaviour, including two-step registration and multiple credential methods; Shield should render the returned nodes rather than hard-code a single registration form.

### 3. Recovery / Password Change / Settings

Shield must treat these as Kratos Self-Service flows instead of inventing its own password API.

```text
/recovery
/settings
```

Examples include account recovery, password changes, profile changes, and MFA configuration. The exact controls exposed by Shield are determined by the Kratos configuration and flow nodes.

### 4. Verification

Verification remains a Kratos flow. Shield renders the verification flow and follows its resulting redirect.

### 5. Hydra Login Challenge

When Hydra redirects the browser to Shield with a `login_challenge`:

```text
GET /oauth2/login?login_challenge=<challenge>
```

Shield must:

1. validate/read the challenge through the Hydra Admin API;
2. determine whether a valid Kratos session already exists;
3. if necessary, start or continue the Kratos login flow;
4. after authentication, resolve the Hydra login challenge through the Admin API;
5. redirect the browser to Hydra's returned redirect URL.

The browser must never call the Hydra Admin API directly.

### 6. Hydra Consent Challenge

When Hydra redirects the browser with a `consent_challenge`:

```text
GET /oauth2/consent?consent_challenge=<challenge>
```

Shield must:

1. retrieve the consent request from Hydra Admin API;
2. identify the authenticated subject from the Hydra login result / Kratos session context;
3. render the client/application identity and requested scopes/claims;
4. optionally support configured consent skipping rules;
5. accept or reject the consent challenge through the Hydra Admin API;
6. redirect the browser to Hydra's returned redirect URL.

## Security Model

### Browser-facing API

Only routes necessary for browser flows are public:

```text
/login
/registration
/recovery
/verification
/settings
/oauth2/login
/oauth2/consent
```

The exact route structure is implementation-defined, but privileged Hydra operations are always server-side.

### Hydra Admin API

`HYDRA_ADMIN_URL` is an internal-only dependency. No browser-generated request may be able to reach it directly.

Shield should use an explicit Hydra client layer rather than exposing a generic reverse proxy such as:

```text
POST /proxy?url=<arbitrary-url>
```

### Credentials

Passwords, recovery codes, TOTP secrets, and other credentials are submitted only to Kratos flows. Shield must not log them, persist them, or copy them into its own data store.

### Session authority

Kratos remains the source of truth for user authentication. Shield should avoid creating a second long-lived authentication session unless a specific browser/UI requirement makes one unavoidable.

## UI Strategy

Shield uses **server-rendered Leptos** for the initial document and can hydrate with WASM for interactive behaviour.

Kratos returns declarative `ui.nodes`. Shield should map those nodes into reusable components rather than hard-code every authentication method into separate forms.

Conceptually:

```text
Kratos UI Node
      |
      +-- input      -> Input
      +-- submit     -> Button
      +-- text       -> Text
      +-- img        -> Image
      +-- a          -> Link
      +-- script     -> controlled client integration
```

Authentication methods such as password, code/OTP, WebAuthn/passkeys, OIDC, and TOTP should be driven primarily by the flow payload and Kratos configuration.

## Rust Stack

- Rust
- Axum
- Leptos (SSR + WASM hydration)
- Tokio
- Reqwest
- Serde
- Tower / tower-http
- `tracing` / `tracing-subscriber`

The first implementation should prefer a small, explicit service layer over a generic proxy abstraction.

Suggested modules:

```text
src/
├── config.rs
├── error.rs
├── main.rs
├── http/
│   ├── routes.rs
│   └── middleware.rs
├── kratos/
│   ├── client.rs
│   ├── flows.rs
│   └── models.rs
├── hydra/
│   ├── client.rs
│   ├── login.rs
│   ├── consent.rs
│   └── models.rs
└── ui/
    ├── app.rs
    ├── components/
    └── pages/
```

## Configuration

| Variable | Description | Default |
|---|---|---|
| `PORT` | Axum listening port | `3000` |
| `PUBLIC_BASE_URL` | Public base URL of Shield | `http://localhost:3000` |
| `KRATOS_PUBLIC_URL` | Kratos Public API URL | `http://kratos-public:4433` |
| `HYDRA_ADMIN_URL` | Hydra Admin API URL; never exposed to browsers | `http://hydra-admin:4445` |
| `RUST_LOG` | tracing filter | `info` |

Kratos and Hydra themselves remain responsible for their own configuration, cookies, allowed return URLs, OAuth clients, and security settings.

## Deployment Model

```text
Internet
   |
   v
Ingress
   |
   v
Shield UI
   | \
   |  \__ Hydra Admin (cluster-internal)
   |
   \____ Kratos Public API
```

Recommended Kubernetes properties:

- Shield has no database in the initial implementation.
- Shield replicas should be interchangeable.
- Hydra Admin is reachable only from the Shield workload/network policy.
- Kratos and Hydra are deployed as separate services.
- Cookies and TLS termination follow the deployment's trusted proxy configuration.

## DiTA Integration

A DiTA portal is only an OIDC client.

```text
1. User selects Login in DiTA.
2. DiTA redirects to Hydra.
3. Hydra redirects to Shield for login.
4. Shield authenticates the user with Kratos.
5. Shield resolves Hydra's login challenge.
6. Shield displays consent when required.
7. Shield resolves Hydra's consent challenge.
8. Hydra redirects to DiTA's OIDC callback.
9. DiTA exchanges the authorization code and establishes its application session.
```

The DiTA application therefore contains **no password-entry UI for the central identity system**.

The same identity provider can later be used by other OIDC clients such as Grafana, Jaeger, or internal services.

## Testing Requirements

The project should include integration tests for at least:

- successful login;
- invalid credentials;
- registration;
- recovery;
- verification;
- password change;
- MFA/TOTP setup and challenge;
- existing Kratos session during Hydra login challenge;
- unauthenticated Hydra login challenge;
- Hydra consent accept;
- Hydra consent reject;
- invalid/expired Hydra challenges;
- invalid `return_to` / redirect scenarios;
- Hydra Admin API unavailable;
- Kratos Public API unavailable;
- CSRF and session-cookie handling.

Tests should exercise the real Kratos/Hydra APIs in a disposable environment rather than relying only on mocked HTTP responses.

## Project Status

This repository is intended to become the maintained browser-facing application for a self-hosted Ory Kratos + Hydra deployment.

The first milestone is **functional correctness of the authentication state machines**, not micro-benchmarks or minimum image size.

Performance claims should only be added after reproducible benchmark results are committed with the benchmark methodology.

## License

MIT
