use crate::hydra::models::ConsentRequest;
use crate::kratos::models::*;
use crate::ui::components::KratosForm;
use leptos::prelude::*;

#[component]
pub fn PageLayout(
    title: &'static str,
    #[prop(optional)] subtitle: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>{title} " - Ory Shield"</title>
                <script src="https://cdn.tailwindcss.com"></script>
                <style>
                    "body {
                        background: radial-gradient(circle at top, #1e293b 0%, #0f172a 100%);
                    }"
                </style>
            </head>
            <body class="text-slate-100 min-height-screen flex flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
                <div class="max-w-md w-full space-y-8 bg-slate-900/60 backdrop-blur-xl border border-slate-800 p-8 rounded-2xl shadow-2xl">
                    <div class="text-center">
                        <div class="mx-auto h-12 w-12 rounded-xl bg-blue-600 flex items-center justify-center shadow-lg shadow-blue-500/30">
                            <span class="text-xl font-bold text-white">"🛡️"</span>
                        </div>
                        <h2 class="mt-6 text-3xl font-extrabold text-white tracking-tight">
                            {title}
                        </h2>
                        {subtitle.map(|sub| view! {
                            <p class="mt-2 text-sm text-slate-400">{sub}</p>
                        })}
                    </div>
                    <div>
                        {children()}
                    </div>
                </div>
            </body>
        </html>
    }
}

#[component]
pub fn LoginPage(flow: LoginFlow) -> impl IntoView {
    // Parse query parameters from flow.request_url to preserve them (e.g. login_challenge, return_to)
    let signup_href = match url::Url::parse(&flow.request_url) {
        Ok(url) => {
            let mut params = vec![];
            for (key, val) in url.query_pairs() {
                if key == "login_challenge" || key == "return_to" {
                    params.push((key.into_owned(), val.into_owned()));
                }
            }
            if params.is_empty() {
                "/registration".to_string()
            } else {
                format!("/registration?{}", serde_urlencoded::to_string(&params).unwrap_or_default())
            }
        }
        Err(_) => "/registration".to_string(),
    };

    view! {
        <PageLayout title="Sign In" subtitle="Sign in to your account".to_string()>
            <div class="mt-6">
                <KratosForm ui=flow.ui />
                // ── Sign-up nudge ────────────────────────────────────────────
                <div class="mt-6 pt-5 border-t border-slate-800 text-center">
                    <p class="text-sm text-slate-400">
                        "Don't have an account? "
                        <a
                            href=signup_href
                            class="font-semibold text-blue-400 hover:text-blue-300 hover:underline transition-colors duration-200"
                        >
                            "Sign up"
                        </a>
                    </p>
                </div>
            </div>
        </PageLayout>
    }
}

#[component]
pub fn RegistrationPage(flow: RegistrationFlow) -> impl IntoView {
    view! {
        <PageLayout title="Create Account" subtitle="Get started with a new account".to_string()>
            <div class="mt-6">
                <KratosForm ui=flow.ui />
            </div>
        </PageLayout>
    }
}

#[component]
pub fn RecoveryPage(flow: RecoveryFlow) -> impl IntoView {
    view! {
        <PageLayout title="Recover Account" subtitle="Enter credentials to recover your account".to_string()>
            <div class="mt-6">
                <KratosForm ui=flow.ui />
            </div>
        </PageLayout>
    }
}

#[component]
pub fn VerificationPage(flow: VerificationFlow) -> impl IntoView {
    view! {
        <PageLayout title="Verify Account" subtitle="Verify your email or account status".to_string()>
            <div class="mt-6">
                <KratosForm ui=flow.ui />
            </div>
        </PageLayout>
    }
}

#[component]
pub fn SettingsPage(flow: SettingsFlow) -> impl IntoView {
    view! {
        <PageLayout title="Account Settings" subtitle="Manage password, profile, and security".to_string()>
            <div class="mt-6">
                <KratosForm ui=flow.ui />
                <div class="mt-6 pt-6 border-t border-slate-800 text-center">
                    <a href="/login" class="text-sm text-slate-400 hover:text-white transition-colors">
                        "Back to Dashboard"
                    </a>
                </div>
            </div>
        </PageLayout>
    }
}

// ─── Error Page ───────────────────────────────────────────────────────────────

/// Renders a self-contained error screen, used by the `GET /error` route.
///
/// * `title`       — Short headline shown prominently (e.g. "Flow Expired").
/// * `description` — Longer explanation shown beneath the title.
/// * `back_url`    — Href for the "Return" button (defaults to `/login`).
#[component]
pub fn ErrorPage(
    title: String,
    description: String,
    #[prop(default = "/login".to_string())] back_url: String,
) -> impl IntoView {
    view! {
        <PageLayout title="Error" subtitle=title.clone()>
            <div class="mt-6 space-y-6">
                // ── Error icon ───────────────────────────────────────────────
                <div class="flex justify-center">
                    <div class="h-14 w-14 rounded-2xl bg-red-900/40 border border-red-700/50 flex items-center justify-center shadow-lg shadow-red-900/20">
                        <span class="text-2xl" role="img" aria-label="Error">
                            "⚠️"
                        </span>
                    </div>
                </div>

                // ── Human-readable description ────────────────────────────────
                <div class="p-4 bg-red-950/30 border border-red-800/40 rounded-xl">
                    <p class="text-sm text-slate-300 leading-relaxed text-center">
                        {description}
                    </p>
                </div>

                // ── Return CTA ───────────────────────────────────────────────
                <div class="pt-2">
                    <a
                        href=back_url
                        class="block w-full py-2.5 px-4 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded-lg shadow-lg hover:shadow-blue-500/20 transition-all duration-200 text-sm text-center"
                    >
                        "Return to Login"
                    </a>
                </div>
            </div>
        </PageLayout>
    }
}

#[component]
pub fn ConsentPage(req: ConsentRequest) -> impl IntoView {
    let client_name = req
        .client
        .client_name
        .unwrap_or_else(|| req.client.client_id.clone());
    let requested_scopes = req.requested_scope.clone();
    let challenge = req.challenge.clone();

    view! {
        <PageLayout title="Authorize Application" subtitle=format!("{} wishes to access your account", client_name)>
            <div class="mt-6 space-y-6">
                <div class="p-4 bg-slate-950/40 border border-slate-800 rounded-xl space-y-3">
                    <h3 class="text-xs font-semibold text-slate-400 uppercase tracking-wider">
                        "Requested Scopes"
                    </h3>
                    <ul class="space-y-2">
                        {requested_scopes.into_iter().map(|scope| view! {
                            <li class="flex items-center space-x-2 text-sm text-slate-200">
                                <span class="text-blue-500">"✓"</span>
                                <span>{scope}</span>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>

                <form method="POST" action="/oauth2/consent" class="space-y-3">
                    <input type="hidden" name="consent_challenge" value=challenge />

                    // Render individual hidden scope inputs so form submission parses them
                    {req.requested_scope.clone().into_iter().map(|scope| view! {
                        <input type="hidden" name="grant_scope[]" value=scope />
                    }).collect::<Vec<_>>()}

                    <div class="grid grid-cols-2 gap-4">
                        <button
                            type="submit"
                            name="submit"
                            value="reject"
                            class="w-full py-2.5 px-4 bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold border border-slate-700 rounded-lg transition-all duration-200 text-sm cursor-pointer"
                        >
                            "Deny"
                        </button>
                        <button
                            type="submit"
                            name="submit"
                            value="accept"
                            class="w-full py-2.5 px-4 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded-lg shadow-lg hover:shadow-blue-500/20 transition-all duration-200 text-sm cursor-pointer"
                        >
                            "Authorize"
                        </button>
                    </div>
                </form>
            </div>
        </PageLayout>
    }
}
