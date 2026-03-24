FROM lewimbes/dioxus AS builder
WORKDIR /app

ARG APP_VER=prod

COPY . .
RUN dx build --release --verbose && \
    mv /app/target/dx/cringe/release/web/ /app.built/ && \
    cp -a /app/migrations/ /app.built/migrations/

FROM gcr.io/distroless/cc
WORKDIR /app

COPY --from=builder /app.built/ /app/

ENV IP=0.0.0.0 \
    PORT=80 \
    APP_VER=$APP_VER
ENTRYPOINT ["/app/cringe"]
