# Chef (Prepare caching environment)
FROM lukemathwalker/cargo-chef:latest-rust-1.97-alpine AS chef
WORKDIR /app

# Planner (Analyze dependencies)
FROM lukemathwalker/cargo-chef:latest-rust-1.97-alpine AS planner
WORKDIR /app
COPY . .
# Compute a recipe file containing all dependencies
RUN cargo chef prepare --recipe-path recipe.json

# Builder (Build dependencies and app)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - This layer is cached efficiently!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Now copy the actual source code
COPY . .
# Build the application statically
RUN cargo build --release --target x86_64-unknown-linux-musl
# Create an unprivileged user for Kubernetes security
RUN adduser -D -g '' -h /nonexistent -s /sbin/nologin -H -u 10001 appuser

# Final minimal image
FROM scratch
# IMPORTANT: Copy CA certificates so the app can make HTTPS requests to Ory Kratos/Hydra
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
# Copy the unprivileged user information
COPY --from=builder /etc/passwd /etc/passwd
# Copy the statically compiled binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ory-selfservice-rust /ory-selfservice-rust
# Switch to the unprivileged user
USER appuser
# Expose the application port
EXPOSE 8080
# Define the entrypoint
CMD ["/ory-selfservice-rust"]
