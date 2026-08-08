# sdkwork-im standalone Docker deployment

Reference implementation: `sdkwork-cloudrouter` standalone container composition
(`docker-compose.yml` + root `Dockerfile`). This directory holds the sdkwork-im
equivalent for `deploymentProfile = "standalone"`, `runtimeTarget = "container"`.

## Layout

- `sdkwork-api-im-standalone-container.Dockerfile` — runtime-only image for the
  `sdkwork-api-im-standalone-gateway` binary plus packaged runtime assets.
- `docker-compose.yml` — standalone composition (PostgreSQL + IM gateway).
  The gateway bootstraps the schema and applies migrations on first start.
- `.env.example` — deployment-time environment template (copy to `.env`).
- `nginx/testimdocker-im.conf` — test-domain vhost (3 domains, WebSocket
  upgrade) for a host-installed nginx.
- `postgres/init/001-create-schema.sh` — creates the workspace schema on first
  PostgreSQL initialization.

## Pipeline

```text
cargo build (Linux) -> scripts/build-im-standalone-container.mjs -> docker build
                                                                    -> docker compose up -d
                                                                    -> nginx vhost -> browser
```

### 1. Build the Linux gateway binary

The gateway depends on sibling SDKWork workspace crates, so it must be built
from the complete workspace (`sdkwork-space`), on Linux/WSL:

```bash
cd /path/to/sdkwork-space/sdkwork-im
CARGO_TARGET_DIR=~/sdkwork-im-target cargo build --release \
  -p sdkwork-api-im-standalone-gateway --bin sdkwork-api-im-standalone-gateway
```

### 2. Build the container image

```bash
pnpm build:container            # -> sdkwork-im-standalone-gateway:local
pnpm build:container:check      # validate the build plan without building
```

The packaging script assembles `dist/container-image-build/`:

- `bin/sdkwork-api-im-standalone-gateway` — the Linux release binary
- `sdkwork.app.config.json` — application identity manifest
- `database/` — IM database module (manifest + migrations)
- `web/sdkwork-im-pc/dist`, `web/sdkwork-im-h5/dist` — renderer static builds
  (built via `pnpm build`; PC renderer uses local gateway discovery at
  `http://127.0.0.1:18079`, H5 renderer defaults to same-origin `/`)
- `modules/<workspace>/database/` — embedded dependency database modules
  (drive, knowledgebase, inventory, invoice, membership, order, payment,
  shop, notary, agents)

### 3. Start the composition

```bash
cd deployments/docker
cp .env.example .env            # optional; defaults are local-development safe
docker compose up -d
docker compose ps               # wait for im-gateway (healthy)
```

The gateway listens on `0.0.0.0:18079` inside the container; the host maps
`18079 -> 18079` so the PC renderer's local gateway discovery keeps working
from browsers over the WSL2 localhost relay.

### 4. nginx (test domains)

Deploy `nginx/testimdocker-im.conf` and `nginx/bootstrap-token.js` (the
credential-entry bootstrap script it injects; see section 4b) to the host
nginx:

```bash
sudo cp deployments/docker/nginx/testimdocker-im.conf /etc/nginx/sites-enabled/
sudo cp deployments/docker/nginx/bootstrap-token.js /etc/nginx/bootstrap-token.js
sudo nginx -t && sudo systemctl reload nginx
```

Windows host resolution (run `cmd`/`powershell` as Administrator):

```text
127.0.0.1 testimdocker.sdkwork.com testimdocker.birdcoder.com testimdocker.dtupay.com
```

Then open `http://testimdocker.sdkwork.com` (desktop UA -> PC renderer,
mobile UA -> H5 renderer on the same origin).

> The `testimdocker.*` test domain family is owned by the standalone IM
> composition on the shared WSL host. Sibling projects use the adjacent
> `testapidocker.*` family (see the archived `testapidocker-api-gateway.conf.orig`
> and the BirdCoder `sdkwork` vhosts); keep vhost `server_name` sets disjoint so
> nginx never routes one product's requests to another.

### 4b. Browser bootstrap Access-Token injection

The renderer SDK reads the development bootstrap Access-Token from
`window.__SDKWORK_CREDENTIAL_ENTRY_BOOTSTRAP_ACCESS_TOKEN__` (or
`process.env.SDKWORK_ACCESS_TOKEN`). The Vite plugin that injects it
(`sdkwork-iam-credential-entry`, vite config `accessToken:
process.env.SDKWORK_ACCESS_TOKEN`) is dev-server-only (`apply: 'serve'`), so
production renderer builds ship without it and the browser login fails with
`access-token-only request requires Access-Token before request dispatch`.

The test nginx fixes this at the deployment layer:

- `nginx/bootstrap-token.js` — sets the well-known development fallback JWT on
  the window global. Deploy next to the vhost
  (`/etc/nginx/bootstrap-token.js`) and serve it with
  `location = /bootstrap-token.js`.
- `nginx/testimdocker-im.conf` — `sub_filter` rewrites the served `index.html`
  to load that script (`<script src="/bootstrap-token.js"></script>`).

It must be an **external same-origin script**: the gateway responds with a
strict CSP (`script-src 'self' 'nonce-...'`), so a plain inline script is
refused by the browser even though it appears in the DOM. `Accept-Encoding ""`
is cleared on the proxy so `sub_filter` can rewrite the body.

Anything real must instead set a signed token via `SDKWORK_ACCESS_TOKEN` at
renderer build time.

### 5. IAM bootstrap (first deployment, once)

The gateway auto-creates the default `admin` user (tenant `100001`), but the
credential-entry login flow needs two more one-time steps:

1. **Admin password credential** — set `SDKWORK_IAM_SUPER_ADMIN_PASSWORD` in
   `.env`, then `docker compose up -d` (the IAM bootstrap provisions/updates
   the password credential for `admin`).
2. **Platform runtime app** — the credential-entry-bootstrap flow requires
   the per-tenant platform app (`app_<tenant_id>`), which no runtime path
   provisions automatically. Run the idempotent SQL once after the gateway
   migrated the IAM tables:

   ```bash
   docker exec -i <postgres-container> psql -U sdkwork_ai_dev -d sdkwork_ai_dev \
     -v ON_ERROR_STOP=1 < deployments/docker/postgres/provision-platform-app.sql
   ```

Login protocol (what the renderer SDK does, verifiable with curl):

```text
POST /app/v3/api/auth/sessions
Header: access-token: <bootstrap access-token JWT>
Body:   {"grantType":"password","username":"admin","password":"<super-admin-password>"}
```

The bootstrap access-token is the unsigned test JWT
(`base64url(header).base64url(payload).test-signature`, `alg:none`) with
claims `token_type=access, tenant_id=100001, user_id=system,
app_id=app_100001, login_scope=TENANT, token_version=1` (the development
fallback). Business API calls use the returned dual tokens:
`Authorization: Bearer <authToken>` + `access-token: <accessToken>` header.

## Configuration notes

- `SDKWORK_CORS_ALLOWED_ORIGINS` is the canonical shared allow-list key
  (SOURCE_CONFIG_SPEC); every public renderer domain plus the local gateway
  port must be listed.
- The PostgreSQL database name must be a canonical SDKWork workspace
  identity (`sdkwork_ai_dev`/`sdkwork_ai_test`/`sdkwork_ai_staging`/
  `sdkwork_ai_prod`) with the matching username; the schema must equal the
  name. `sdkwork_ai_dev` pairs with `SDKWORK_IM_ENVIRONMENT=development`.
- PostgreSQL runs the `pgvector/pgvector:pg16` image: the embedded
  knowledgebase database module requires the `vector` extension at migration
  time (plain `postgres:16-alpine` cannot provide it).
- `SDKWORK_IM_ENVIRONMENT=development` enables the IAM credential-entry login
  flow with the gateway-issued bootstrap Access-Token. For anything real,
  set a production environment and provide
  `SDKWORK_IAM_SUPER_ADMIN_PASSWORD` (or the IAM signing master secret).
- Redis is not required by the standalone gateway
  (`SDKWORK_IM_REDIS_ENABLED` defaults to false); the composition is
  PostgreSQL-only.
- PostgreSQL data lives in the `im-postgres-data` named volume; `docker
  compose down` keeps it, `docker compose down -v` destroys it.
- Changing `SDKWORK_IM_POSTGRES_DB` also changes the schema name (the init
  script derives it from `POSTGRES_DB`); keep
  `SDKWORK_IM_POSTGRES_SCHEMA` equal to it.

## Troubleshooting (WSL deployments)

- WSL often exports `http_proxy`/`https_proxy` to a Windows proxy
  (e.g. `127.0.0.1:7897`). Any curl test against the domains/ports will then
  return `502 Bad Gateway` from the proxy instead of reaching nginx — use
  `curl --noproxy '*' ...` or add the domains to `no_proxy`.
- Git Bash (MSYS2) curl resolves hostnames against its own
  `C:/Program Files/Git/etc/hosts`, not the Windows hosts file — add the
  domains there too, or use `curl.exe`/a browser.
- If `sudo nginx -s reload` reports `open() "/run/nginx.pid" failed`, the
  master pid file was lost (WSL `/run`); recreate it
  (`echo <master-pid> | sudo tee /run/nginx.pid`) and reload.
- The nginx worker (`www-data`) must be able to create the vhost log files:
  `sudo touch /var/log/nginx/testimdocker-im.access.log /var/log/nginx/testimdocker-im.error.log
  && sudo chown www-data:adm /var/log/nginx/testimdocker-im.*.log`.
