# VPS Commands Reference

## 1) Fresh deploy

```bash
cd ~
rm -rf insiderlist
git clone git@github.com:itsarraj/insiderlist.git
cd ~/insiderlist
docker-compose up --build -d
docker-compose ps
```

## 2) Update existing deployment

```bash
cd ~/insiderlist
git pull
docker-compose up --build -d
docker-compose ps
```

## 3) Logs

```bash
docker-compose logs --tail=100 api
docker-compose logs --tail=100 web
docker-compose logs --tail=100 postgres
docker-compose logs -f api web postgres
```

## 4) API tests (direct on VPS)

```bash
curl -i http://localhost:8080/api/v1/health

curl -i -X POST "http://localhost:8080/api/v1/subscribe" \
  -H "Content-Type: application/json" \
  -d '{"email":"not-an-email"}'

curl -i -X POST "http://localhost:8080/api/v1/subscribe" \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com"}'
```

## 5) API tests (through public domains)

```bash
curl -i https://seekersconnect.xyz/api/v1/health
curl -i https://matchmyresume.xyz/api/v1/health
```

## 6) Database checks

```bash
docker-compose exec postgres psql -U insiderlist -d insiderlist -c \
"SELECT email, created_at FROM newsletter_subscribers ORDER BY created_at DESC LIMIT 10;"
```

## 7) Nginx checks (host)

```bash
sudo nginx -t
sudo nginx -T | rg "server_name|listen|conf.d"
sudo systemctl reload nginx
curl -I -H "Host: seekersconnect.xyz" http://127.0.0.1
curl -I -H "Host: matchmyresume.xyz" http://127.0.0.1
```

## 8) HTTPS (certbot)

```bash
sudo certbot --nginx \
  -d seekersconnect.xyz -d www.seekersconnect.xyz \
  -d matchmyresume.xyz -d www.matchmyresume.xyz

sudo certbot renew --dry-run
```

## 9) DNS/firewall verification

```bash
dig +short seekersconnect.xyz A
dig +short matchmyresume.xyz A
sudo ufw status
```

## 10) Recovery / restart

```bash
cd ~/insiderlist
docker-compose restart api web
docker-compose down
docker-compose up --build -d
docker-compose ps
```

