FROM lewimbes/dioxus AS builder
WORKDIR /app

ARG TARGET3=x86_64-unknown-linux-musl

RUN set -eux; \
    if [[ "$TARGET3 " == '*musl ']]; then \
        apt-get update; \
        apt-get install -y --no-install-recommends musl-tools; \
        rm -rf /var/lib/apt/lists/*; \
    fi; \
    rustup target add "$TARGET3"

COPY . .
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-target,target=/app/target \
    set -eux; \
    if echo "$TARGET3" | grep -q 'musl'; then \
        export CC_x86_64_unknown_linux_musl=musl-gcc; \
        export CC_aarch64_unknown_linux_musl=musl-gcc; \
    fi; \
    dx build --release --verbose --debug-symbols=false --fullstack \
        @server \
            --target "$TARGET3" \
            --no-default-features \
            --features server && \
    mv /app/target/dx/cringe/release/web /app.built && \
    cp /app/target/"$TARGET3"/server-release/cringe /app.built/cringe

FROM scratch
WORKDIR /app

COPY --from=builder /app.built/ /app/
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENTRYPOINT ["/app/cringe"]
