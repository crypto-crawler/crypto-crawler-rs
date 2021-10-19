# CarbonBot

A CLI tool to crawl trade, level2 orderbook updates and snapshots, ticker, funding rate, etc.

## Run

```bash
# trade
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.trade.config.js

# level2 orderbook updates
docker run -d --name carbonbot-l2_event --restart always -v $YOUR_LOCAL_PATH:/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.l2_event.config.js

# level2 orderbook snapshots
docker run -d --name carbonbot-l2_snapshot --restart always -v $YOUR_LOCAL_PATH:/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.l2_snapshot.config.js

# ticker
docker run -d --name carbonbot-ticker --restart always -v $YOUR_LOCAL_PATH:/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.ticker.config.js

# funding_rate
docker run -d --name carbonbot-funding_rate --restart always -v $YOUR_LOCAL_PATH:/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.funding_rate.config.js

# other
docker run -d --name carbonbot-other --restart always -v $YOUR_LOCAL_PATH:/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.other.config.js
```

**Environment Variables**:

| Name                  | Required | Meaning                                                       |
| --------------------- | -------- | ------------------------------------------------------------- |
| AWS_ACCESS_KEY_ID     | true     | AWS access key ID                                             |
| AWS_SECRET_ACCESS_KEY | true     | AWS secret access key                                         |
| AWS_S3_DIR            | true     | AWS S3 destination path                                       |
| PARSER                | false    | If true, data will parsed by the library `crypto_msg_parser`  |
| REDIS_URL             | false    | If set to non-empty, data will be published to redis channels |

The `soulmachine/carbonbot` container writes data to local path first, and then uploads to AWS S3 every 15 minutes.

## Build

```bash
docker pull ghcr.io/rust-lang/rust:nightly-bullseye-slim && docker pull node:bullseye-slim
docker build -t soulmachine/carbonbot:debian -f Dockerfile.debian .
docker push soulmachine/carbonbot:debian

docker build -t soulmachine/carbonbot:amazonlinux -f Dockerfile.amazonlinux .
docker push soulmachine/carbonbot:amazonlinux
```

## Download

| File Name                | MD5                              | Size        | Magnet Link                                                                                                                                                                                                                                                    |
| ------------------------ | -------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| parsed-trade-2021-07.zip | a4a4088c3c9ebccc70e4b10f77c044c3 | 84213736076 | magnet:?xt=urn:btih:557afe1132dd5a67dada971009733ae6019fd84b&dn=parsed-trade-2021-07.zip&tr=http%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=http%3A%2F%2Fp4p.arenabg.com%3A1337%2Fannounce |
