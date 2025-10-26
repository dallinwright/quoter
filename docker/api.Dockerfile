# Multi-stage build for Rust project using Alpine Linux
FROM rust:alpine3.21 AS builder

# Arguments for platform and username
ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG PACKAGE_NAME

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev build-base pkgconfig openssl-libs-static git perl

# Set the working directory
WORKDIR /src

# Create a target directory to avoid permission issues
RUN mkdir -p ./target

# Copy project files
COPY .cargo/config.toml /.cargo/config.toml
COPY src /src/
COPY bins /bins/
COPY crates /crates/
COPY Cargo.toml /Cargo.toml

RUN --mount=type=secret,id=github \
    mkdir -p /root && \
    cp /run/secrets/github /root/.git-credentials && \
    echo >> /root/.git-credentials && \
    git config --global credential.helper store && \
    cargo build --release \
        --package $PACKAGE_NAME \
        --bin $PACKAGE_NAME

# Runner stage with a minimal base image
FROM rust:alpine3.21 AS runner

ARG PACKAGE_NAME

COPY --from=builder /target/release/$PACKAGE_NAME /usr/local/bin/

RUN mv /usr/local/bin/$PACKAGE_NAME /usr/local/bin/app

# Create a new user for running the service
RUN addgroup -S appuser && adduser -S appuser -G appuser

# Set ownership and permissions
RUN chown appuser:appuser /usr/local/bin/app \
    && chmod 755 /usr/local/bin/app

# Switch to the non-root user
USER appuser

EXPOSE 8080

# Set the entrypoint and command
ENTRYPOINT ["/usr/local/bin/app"]
