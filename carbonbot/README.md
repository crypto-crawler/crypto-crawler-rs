# CarbonBot

A CLI tool to crawl trade, level2 orderbook updates and snapshots, ticker, funding rate, etc.

## Run

To quickly get started, copy `conf/run_crawlers.sh` to somewhere, fill in neccesary parameters and run it.

### Trade

Crawl tick-by-tick trades:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.trade.config.js
```

### Level2 orderbook updates

Crawl realtime level2 orderbook incremental updates:

```bash
docker run -d --name carbonbot-l2_event --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.l2_event.config.js
```

### Level2 orderbook full snapshots from RESTful API

Crawl level2 orderbook full snapshots from RESTful API:

```bash
docker run -d --name carbonbot-l2_snapshot --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.l2_snapshot.config.js
```

### Level2 orderbook top-k snapshots

Crawl realtime level2 orderbook top-K snapshots:

```bash
docker run -d --name carbonbot-l2_topk --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.l2_topk.config.js
```

### Level3 orderbook updates

Crawl realtime level3 orderbook incremental updates:

```bash
docker run -d --name carbonbot-l3_event --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.l3_event.config.js
```

### BBO

Crawl realtime BBO:

```bash
docker run -d --name carbonbot-bbo --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.bbo.config.js
```

### Ticker

Crawl 24hr rolling window tickers:

```bash
docker run -d --name carbonbot-ticker --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.ticker.config.js
```

### Candlestick

Crawl candlesticks(i.e., OHLCV)

```bash
docker run -d --name carbonbot-candlestick --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.candlestick.config.js
```

### Funding rate

Crawl funding rates

```bash
docker run -d --name carbonbot-funding_rate --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.funding_rate.config.js
```

### Open interest

```bash
docker run -d --name carbonbot-open_interest --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.open_interest.config.js
```

### Other

```bash
docker run -d --name carbonbot-other --restart always -v $YOUR_LOCAL_PATH:/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.other.config.js
```

**Environment Variables**:

| Name                  | Required | Meaning                                                       |
| --------------------- | -------- | ------------------------------------------------------------- |
| DATA_DIR              | true     | The local directory to store data                             |
| AWS_ACCESS_KEY_ID     | true     | AWS access key ID                                             |
| AWS_SECRET_ACCESS_KEY | true     | AWS secret access key                                         |
| AWS_S3_DIR            | true     | AWS S3 destination path                                       |
| REDIS_URL             | false    | If set to non-empty, data will be published to redis channels |

The `soulmachine/carbonbot` container writes data to the local path temporarily, then moves data to AWS S3 every 15 minutes.

## Build

```bash
docker pull ghcr.io/rust-lang/rust:nightly-bullseye-slim && docker pull node:bullseye-slim
docker build -t soulmachine/carbonbot .
docker push soulmachine/carbonbot
```

## Download

| File Name                | MD5                              | Size        | Magnet Link                                                                                                                                                                                                                                                    |
| ------------------------ | -------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| parsed-trade-2021-07.zip | a4a4088c3c9ebccc70e4b10f77c044c3 | 84213736076 | magnet:?xt=urn:btih:557afe1132dd5a67dada971009733ae6019fd84b&dn=parsed-trade-2021-07.zip&tr=http%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=http%3A%2F%2Fp4p.arenabg.com%3A1337%2Fannounce |
