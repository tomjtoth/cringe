FROM lewimbes/dioxus AS builder
WORKDIR /app

# needed for local dev, GHA overrides this with arm64
ARG TARGETARCH=amd64

RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools && \
    rm -rf /var/lib/apt/lists/*

RUN case "$TARGETARCH" in \
        amd64) echo x86_64-unknown-linux-musl ;; \
        arm64) echo aarch64-unknown-linux-musl ;; \
    esac > /rust-target && \
    rustup target add "$(cat /rust-target)"

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    RUST_TARGET=$(cat /rust-target) && \
    CC_x86_64_unknown_linux_musl=musl-gcc \
    CC_aarch64_unknown_linux_musl=musl-gcc \
    dx build --release --verbose --debug-symbols=false --fullstack \
        @server \
            --target "$RUST_TARGET" \
            --no-default-features \
            --features server && \
    mv /app/target/dx/cringe/release/web /app.built && \
    cp /app/target/"$RUST_TARGET"/server-release/cringe /app.built/cringe

FROM scratch
WORKDIR /app

COPY --from=builder /app.built/ /app/
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENTRYPOINT ["/app/cringe"]
