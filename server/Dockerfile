# Use the official Rust image.
FROM rust:slim-bookworm as builder

# Install OpenSSL, pkg-config, and PostgreSQL client libraries.
RUN apt-get update && apt-get install -y libpq-dev pkg-config

WORKDIR /usr/src/todo
RUN cargo init

# Copy the Cargo manifest files and build a dummy application.
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the actual source code and rebuild the application.
COPY ./src ./src
RUN touch src/main.rs && cargo build --release

# Final stage: create the runtime image
FROM debian:bookworm-slim

# Install OpenSSL (runtime version) and PostgreSQL client libraries.
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -U appuser

# Copy binary and files
COPY --from=builder /usr/src/todo/target/release/todo /usr/local/bin/

# Set the working directory and command
WORKDIR /usr/local/bin
RUN mkdir /usr/local/bin/logs && chown appuser:appuser /usr/local/bin/logs

USER appuser
CMD ["todo"]