#!/usr/bin/env bash
set -e

ENV_FILE="$(dirname "$0")/.env"

JWT_SECRET=$(openssl rand -hex 32)

cat > "$ENV_FILE" <<EOF
DATABASE_URL=postgres://hackari:hackari_dev@localhost:5432/hackari

JWT_SECRET=${JWT_SECRET}
JWT_ACCESS_EXPIRY=15m
JWT_REFRESH_EXPIRY=7d

AWS_ACCESS_KEY_ID=minioadmin
AWS_SECRET_ACCESS_KEY=minioadmin
AWS_REGION=us-east-1
AWS_ENDPOINT=http://localhost:9000
AWS_BUCKET_NAME=hackari

REDIS_URL=redis://127.0.0.1:6379

SMTP_USER=didorenkoalexander
SMTP_PASSWORD=fsvnzgnvmcfbmwsq
FROM_EMAIL=didorenkoalexander@yandex.ru
FRONTEND_URL=http://111.88.149.106:5173

RUST_LOG=debug,tower_http=debug
EOF

echo ".env сгенерирован (JWT_SECRET: ${JWT_SECRET})"
