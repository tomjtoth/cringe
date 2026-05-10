# Cringe

Just a Hinge clone with additional fictional characters, deployed [here](https://cringe.ttj.hu).

## Development

This app is built with Dioxus - [install its cli tool and deps](https://dioxuslabs.com/learn/0.7/getting_started); relies on OAuth for authentication - define both `AUTH_<PROVIDER>_{ID,CLIENT}` environment variables for each [provider](./src/auth/providers.rs#L18-L48); and uses postgres for storage - also define `DATABASE_URL=postgres:// ...`. I recommend creating a `.env` file.

Start postgres in a dedicated terminal and pass env vars you used for building the DATABASE_URL:

```sh
docker run -it --rm -p 5432:5432 \
    -e POSTGRES_USER=cringe \
    -e POSTGRES_PASSWORD=cringe \
    -e POSTGRES_DB=cringe \
    postgres:alpine
```

Then launch the app with the below command:

```sh
dx serve
```

For Docker image testing:

```sh
docker build . -t test

# the app image is read from env var $IMG
IMG=test docker compose up
```

### OAuth

When testing providers requiring TLS:

```sh
# launch the app
REDIRECT_URL=https://127.0.0.1:8080/api/auth/callback dx serve --port 3000

# launch caddy
caddy reverse-proxy --disable-redirects \
    --from https://127.0.0.1:8080 --to http://127.0.0.1:3000
```

Visit the app [login view](https://127.0.0.1:8080/login).
