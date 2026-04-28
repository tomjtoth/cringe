FROM lewimbes/dioxus:0.7.6 AS builder
WORKDIR /app

COPY . .

# to speed up the occasional local debugging
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-target,target=/app/target \
    dx build --release --verbose --debug-symbols=false && \
    mv /app/target/dx/cringe/release/web/ /app.built/ && \
    cp -a /app/migrations/ /app.built/migrations/

FROM gcr.io/distroless/cc
WORKDIR /app

COPY --from=builder /app.built/ /app/

ENV IP=0.0.0.0 \
    PORT=80

ENTRYPOINT ["/app/server"]
