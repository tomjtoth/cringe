FROM lewimbes/dioxus AS builder
WORKDIR /app

ARG SERVER_TRIPLET=x86_64-unknown-linux-musl

RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools && \
    rm -rf /var/lib/apt/lists/* && \
    rustup target add $SERVER_TRIPLET

COPY . .
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-target,target=/app/target \
    CC_x86_64_unknown_linux_musl=musl-gcc \
    CC_aarch64_unknown_linux_musl=musl-gcc \
    dx build --release --verbose --debug-symbols=false \
        @server --target $SERVER_TRIPLET && \
    mv /app/target/dx/cringe/release/web /app.built && \
    cp /app/target/$SERVER_TRIPLET/server-release/cringe /app.built/cringe && \
    cp -a /app/migrations/ /app.built/migrations/

FROM scratch
WORKDIR /app

COPY --from=builder /app.built/ /app/
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ARG APP_VER=prod
ENV IP=0.0.0.0 \
    PORT=80 \
    APP_VER=$APP_VER \
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENTRYPOINT ["/app/cringe"]
