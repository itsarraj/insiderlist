# InsiderList VPS Deployment Runbook

This is the complete production deployment runbook for this repository using:

- Docker Compose app stack (`postgres`, `api`, `web`)
- Host nginx as edge reverse proxy on `:80/:443`
- Let's Encrypt certificates via Certbot

It includes first-time setup, updates, validation, troubleshooting, and rollback.

## 0) Architecture and traffic flow

```text
Browser
  -> domains (seekersconnect.xyz / matchmyresume.xyz)
  -> VPS host nginx (:80/:443)
  -> proxy_pass 127.0.0.1:8080
  -> Docker container "web" (nginx static frontend)
  -> /api/v1/* proxied by web container to "api:8000"
  -> api talks to postgres:5432
```

Notes:
- Host nginx is public entrypoint.
- Docker `web` is app web server and API gateway to `api`.
- API endpoints are versioned under `/api/v1`.

## 1) Prerequisites checklist

### 1.1 DNS

Create A records pointing to your VPS IP (`45.154.197.90`):

- `seekersconnect.xyz`
- `www.seekersconnect.xyz`
- `matchmyresume.xyz`
- `www.matchmyresume.xyz`

Verify:

```bash
dig +short seekersconnect.xyz A
dig +short www.seekersconnect.xyz A
dig +short matchmyresume.xyz A
dig +short www.matchmyresume.xyz A
```

All should return your VPS IP.

### 1.2 Firewall / network

Open inbound ports:
- `22/tcp` (SSH)
- `80/tcp` (HTTP)
- `443/tcp` (HTTPS)

If using UFW:

```bash
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw status
```

### 1.3 Required software on VPS

- `git`
- `docker` and `docker-compose`
- `nginx`
- `certbot` + nginx plugin

## 2) First-time app deployment

```bash
cd ~
rm -rf insiderlist
git clone git@github.com:itsarraj/insiderlist.git
cd insiderlist
docker-compose up --build -d
docker-compose ps
```

Expected service state:
- `postgres`: `healthy`
- `api`: `up`
- `web`: `up` with host mapping `0.0.0.0:8080->80/tcp`

If build is slow the first time, this is normal.

## 3) Verify app directly before host nginx

```bash
curl -I http://127.0.0.1:8080
curl -i http://127.0.0.1:8080/api/v1/health
```

Important:
- `POST /api/subscribe` is wrong and returns `404`.
- Correct path is `POST /api/v1/subscribe`.

## 4) Configure host nginx reverse proxy

Create `/etc/nginx/conf.d/insiderlist.conf`:

```nginx
server {
    listen 80;
    server_name seekersconnect.xyz www.seekersconnect.xyz matchmyresume.xyz www.matchmyresume.xyz;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

Then validate and reload:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

If you still see "Welcome to nginx!", default server config is taking precedence.

## 5) Validate host nginx routing

From VPS:

```bash
curl -I -H "Host: seekersconnect.xyz" http://127.0.0.1
curl -I -H "Host: matchmyresume.xyz" http://127.0.0.1
```

From local machine:

```bash
curl -I http://seekersconnect.xyz
curl -I http://matchmyresume.xyz
```

Both should return `200`.

## 6) Configure HTTPS with Certbot

Install Certbot + nginx plugin (distro-specific command), then run:

```bash
sudo certbot --nginx \
  -d seekersconnect.xyz -d www.seekersconnect.xyz \
  -d matchmyresume.xyz -d www.matchmyresume.xyz
```

Certbot will:
- issue certificates
- edit nginx config to add TLS server blocks
- configure HTTP->HTTPS redirects if selected

Verify:

```bash
curl -I https://seekersconnect.xyz
curl -I https://matchmyresume.xyz
```

Test renewal:

```bash
sudo certbot renew --dry-run
```

## 7) Application validation suite

### 7.1 Health endpoint

```bash
curl -i https://seekersconnect.xyz/api/v1/health
```

### 7.2 Subscribe endpoint

Invalid payload should be `400`:

```bash
curl -i -X POST "https://seekersconnect.xyz/api/v1/subscribe" \
  -H "Content-Type: application/json" \
  -d '{"email":"not-an-email"}'
```

Valid payload should be `200`:

```bash
curl -i -X POST "https://seekersconnect.xyz/api/v1/subscribe" \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com"}'
```

### 7.3 Database verification

```bash
docker-compose exec postgres psql -U insiderlist -d insiderlist -c \
"SELECT email, created_at FROM newsletter_subscribers ORDER BY created_at DESC LIMIT 20;"
```

### 7.4 Logs

```bash
docker-compose logs --tail=200 api
docker-compose logs --tail=200 web
docker-compose logs --tail=200 postgres
```

## 8) Day-2 operations

### 8.1 Update deployment

```bash
cd ~/insiderlist
git pull
docker-compose up --build -d
docker-compose ps
```

### 8.2 Restart specific services

```bash
cd ~/insiderlist
docker-compose restart api web
```

### 8.3 Full rebuild

```bash
cd ~/insiderlist
docker-compose down
docker-compose up --build -d
```

## 9) Known gotchas and fixes

### 9.1 API panic: `missing configuration field "database"`

Cause: wrong environment variable format in Compose.

This app reads env with prefix `APP` and separator `__`.
Use keys like:
- `APP__DATABASE__HOST`
- `APP__DATABASE__PORT`
- `APP__DATABASE__USERNAME`
- `APP__DATABASE__PASSWORD`
- `APP__DATABASE__DATABASE_NAME`

### 9.2 `404 Not Found` for subscribe endpoint

Wrong URL. Use:
- `POST /api/v1/subscribe`

### 9.3 Web fails with `host not found in upstream "api"`

Usually happens if `api` crashed and web nginx cannot resolve it at startup.

Check:
```bash
docker-compose logs --tail=200 api
docker-compose ps
```

### 9.4 "Welcome to nginx!" appears on domain

Host nginx default server is still active.

Check loaded config:
```bash
sudo nginx -T | rg "server_name|listen|conf.d"
```

Ensure your vhost has the correct `server_name` and default welcome block is not matching the request.

### 9.5 Certbot challenge fails

Usually DNS/firewall/proxy issue.

Checklist:
- A records point to VPS IP
- ports `80/443` are open
- domain is reachable over plain HTTP
- Cloudflare proxy disabled temporarily (DNS-only) if challenge is blocked

## 10) Rollback procedure

If a deployment fails:

1) Revert code:
```bash
cd ~/insiderlist
git log --oneline -n 5
git checkout <last-known-good-commit>
```

2) Redeploy:
```bash
docker-compose up --build -d
docker-compose ps
```

3) Validate:
```bash
curl -i http://127.0.0.1:8080/api/v1/health
```

If you prefer branch-based rollback, checkout the previous release tag/branch instead of a raw commit.

## 11) Security and reliability recommendations

- Keep secrets out of git; use env files or secret manager.
- Rotate SSH keys and disable password auth if possible.
- Enable unattended security updates for OS packages.
- Add external uptime checks on:
  - `https://seekersconnect.xyz/`
  - `https://seekersconnect.xyz/api/v1/health`
- Back up Postgres volume (`pgdata`) regularly.
- Consider adding Docker restart policies and API/web healthchecks in Compose.

