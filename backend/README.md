# InsiderList API

Postgres-backed waitlist API: `POST /api/v1/subscribe` with JSON `{ "email": "..." }`, and `GET /api/v1/health`.

## Local run

1. Start PostgreSQL and create DB/user matching `configuration.yaml` (see `configuration.yaml.example`).
2. `cargo run` from this directory (migrations run on startup).

## Configuration

- File: `configuration.yaml` (optional if every key is set via environment).
- Environment: prefix `APP__`, nested keys use `__`, e.g. `APP_DATABASE__HOST`, `APP_EMAIL__API_KEY`.

Optional transactional email uses [Resend](https://resend.com); leave `email.api_key` empty to disable.

`POST /api/v1/subscribe` is rate-limited per client IP (see `rate_limit` in config). The limiter uses `X-Forwarded-For` / `X-Real-IP` when present (e.g. behind nginx). That stops simple scripted floods from one network; sustained abuse from many IPs still needs a CAPTCHA (e.g. Cloudflare Turnstile) or edge protection (WAF / Cloudflare).

## Docker

Use the repository root `docker-compose.yml` (builds API + static UI + Postgres).

## VPS deployment docs

- Full guide: `docs/VPS_DEPLOYMENT.md`
- Command quick reference: `docs/VPS_COMMANDS_REFERENCE.md`
