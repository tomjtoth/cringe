# Cringe

Just a Hinge clone with additional fictional characters, deployed [here](https://cringe.ttj.hu).

## Development

This app is built with Dioxus - [install its cli tool and deps](https://dioxuslabs.com/learn/0.7/getting_started); relies on OAuth for authentication - define both `AUTH_<PROVIDER>_{ID,CLIENT}` environment variables for each [provider](./src/auth/providers.rs#L18-L48); and uses postgres for storage - also define `DATABASE_URL=postgres:// ...`

I recommend creating `.env` in the project root.

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

Navigate to the [login view](https://127.0.0.1:8080/login).

## Roadmap

Tagged commits are also built as Docker images ([tomjtoth/cringe:\<VERSION\>](https://hub.docker.com/r/tomjtoth/cringe/tags)). Minor releases equal _approximately_ to the following milestones:

- [`0.1`](https://github.com/tomjtoth/cringe/releases/tag/v0.1.5) static website installable as PWA
- [`0.2`](https://github.com/tomjtoth/cringe/releases/tag/v0.2.20) + backend
- [`0.3`](https://github.com/tomjtoth/cringe/releases/tag/v0.3.1) + authentication
- [`0.4`](https://github.com/tomjtoth/cringe/releases/tag/v0.4.3) + prompt editor
- [`0.5`](https://github.com/tomjtoth/cringe/releases/tag/v0.5.4) + details editor
- [`0.6`](https://github.com/tomjtoth/cringe/releases/tag/v0.6.10) + image editor
- [`0.7`](https://github.com/tomjtoth/cringe/releases/tag/v0.7.13) + live profile updates
- [`0.8`](https://github.com/tomjtoth/cringe/releases/tag/v0.8.1) + profile filtering

TODO:

- `0.9` + showcase **bots'** profiles w/o login, tinder.com style
- `0.10` + matching, messaging & un-matching
- `0.11` + display humans' shared skips/likes of bots' profiles

- `1.0` code cleanup
