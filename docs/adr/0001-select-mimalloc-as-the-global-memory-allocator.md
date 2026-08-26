# 0001. Select mimalloc as the Global Memory Allocator

Date: 2026-08-26

## Status

Accepted

## Context

The application currently uses the default memory allocator provided by the `musl`-based build environment.

The application is deployed as a statically linked Linux binary using the `x86_64-unknown-linux-musl` target and a `scratch` runtime image. This deployment model is intentionally retained because it provides a very small runtime image and removes the need for a full userspace distribution in the final container.

The application is an asynchronous Rust service built on Tokio and Axum. Its workload consists primarily of HTTP request handling, serialization and deserialization, interaction with Ory Kratos and Ory Hydra, URL processing, and other request-scoped allocations. It is therefore desirable to use a general-purpose allocator with good performance and memory behavior without introducing unnecessary build and deployment complexity.

Two alternative allocators were considered:

* `jemalloc`, primarily through the Rust `tikv-jemallocator` crate.
* `mimalloc`, through the Rust `mimalloc` crate.

`jemalloc` is a mature allocator with extensive production usage, strong concurrency characteristics, and significant tuning and profiling capabilities. The Rust Analyzer project also selected jemalloc for its release artifacts, demonstrating its suitability for allocation-intensive Rust workloads.

However, the current `tikv-jemallocator` 0.7.x release has a known build problem for the `x86_64-unknown-linux-musl` target. The project's issue tracker documents that version 0.7.0 no longer builds for this target because `background_threads_runtime_support` is not supported on musl, while 0.6.1 still works.

Using an older jemalloc integration solely to retain the musl/scratch deployment model would introduce additional version constraints and build complexity. Using a glibc-based build instead would make a `scratch` runtime impractical without additionally supplying the dynamic loader and glibc runtime libraries.

`mimalloc` provides a Rust global allocator wrapper with a simple integration model:

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

The current `mimalloc` crate release is `0.1.52`. It requires a C compiler during the build and uses mimalloc v3 by default.

The combination of `mimalloc`, static musl linking, and a `scratch` runtime therefore provides a simpler deployment path while still replacing the default allocator with a dedicated general-purpose allocator.

## Decision

The application will use **mimalloc as its global memory allocator**.

The `mimalloc` crate will be added as a dependency:

```toml
[dependencies]
mimalloc = "0.1"
```

The allocator will be installed as the process-wide Rust global allocator:

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

The application will continue to use:

* `x86_64-unknown-linux-musl` for static Linux builds.
* `musl` as the C standard library.
* `scratch` as the final container image.
* A statically linked executable as the only application binary in the runtime image.

The default `mimalloc` configuration will be used initially. The `secure` feature will not be enabled by default because it introduces additional runtime overhead and is not currently required by the application's threat model. The `mimalloc` documentation states that secure mode adds guard pages, randomized allocation, encrypted free lists, and other protections, with an approximately 10% performance penalty according to its own benchmarks.

The Docker builder must provide a C compiler because the `mimalloc` crate builds the native allocator implementation as part of the Rust build.

## Consequences

### Positive

The application retains a fully static musl-based binary and therefore remains compatible with a `scratch` runtime image.

The final container remains extremely small and contains no unnecessary operating-system userspace.

Allocator integration is straightforward and does not require jemalloc-specific configuration, profiling interfaces, or allocator tuning.

The application is no longer dependent on the default allocator behavior of the musl-based build environment.

The chosen allocator is actively maintained; the current Rust crate release is `mimalloc 0.1.52`, and it uses mimalloc v3 by default.

The deployment architecture remains consistent across development and production without introducing a glibc runtime solely to accommodate the allocator choice.

### Negative

`mimalloc` requires a C compiler during the build, which slightly increases the requirements of the builder image.

The application will not have the same allocator-specific tuning and introspection capabilities that jemalloc provides.

The allocator choice is not guaranteed to be optimal for every possible future workload. If the service develops a substantially different allocation pattern, allocator performance should be reevaluated using application-specific benchmarks rather than relying solely on generic allocator benchmarks.

If future requirements justify using jemalloc, the musl compatibility of the selected Rust jemalloc integration must be verified before adoption.

The final `scratch` image does not provide a general userspace environment. Required runtime files such as CA certificates and user information must therefore continue to be explicitly copied into the image.
