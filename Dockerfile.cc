FROM lewimbes/dioxus AS builder
WORKDIR /app

COPY . .
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-target,target=/app/target \
    dx build --release --verbose && \
    mv /app/target/dx/cringe/release/web/ /app.built/ && \
    cp -a /app/migrations/ /app.built/migrations/

FROM gcr.io/distroless/cc
WORKDIR /app

# allow overriding at build time without affecting the builder stage cache
ARG APP_VER=prod
COPY --from=builder /app.built/ /app/

ENV IP=0.0.0.0 \
    PORT=80 \
    APP_VER=$APP_VER
    
ENTRYPOINT ["/app/cringe"]
