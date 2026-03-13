FROM lewimbes/dioxus AS builder
WORKDIR /app

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    dx build --release --verbose --debug-symbols=false && \
    mv /app/target/dx/cringe/release/web /app.built

FROM scratch
WORKDIR /app

COPY --from=builder /app.built /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENTRYPOINT ["/app/cringe"]
