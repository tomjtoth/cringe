# Cringe

Just a Hinge clone with the additional movie stars deployed [here](https://cringe.ttj.hu).

## Development

For the ad-hoc image testing:

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
sudo caddy reverse-proxy --from https://127.0.0.1:8080 --to http://127.0.0.1:3000
```

Visit the app [login view](https://127.0.0.1:8080/login).
