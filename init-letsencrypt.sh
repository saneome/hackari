#!/bin/sh
set -eu

: "${DOMAIN:?DOMAIN is required}"
: "${CERTBOT_EMAIL:?CERTBOT_EMAIL is required}"

STAGING_ARG=""
if [ "${CERTBOT_STAGING:-0}" = "1" ]; then
  STAGING_ARG="--staging"
fi

docker compose run --rm --entrypoint sh certbot -c "mkdir -p /etc/letsencrypt/live/$DOMAIN && \
openssl req -x509 -nodes -newkey rsa:2048 -days 1 \
-keyout /etc/letsencrypt/live/$DOMAIN/privkey.pem \
-out /etc/letsencrypt/live/$DOMAIN/fullchain.pem \
-subj '/CN=$DOMAIN'"

docker compose up -d nginx

docker compose run --rm --entrypoint certbot certbot certonly \
  --webroot \
  --webroot-path /var/www/certbot \
  --email "$CERTBOT_EMAIL" \
  --agree-tos \
  --no-eff-email \
  --force-renewal \
  $STAGING_ARG \
  -d "$DOMAIN"

docker compose exec nginx nginx -s reload
