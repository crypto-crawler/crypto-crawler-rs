# CarbonBot

## Which trading pairs/symbols are chosen?

-   All pairs/symbols in contract markets, including futures, perpetual swap and options.
-   For pairs/symbols in spot market, all of the following criteria must be met:

    -   Base symbol must be in top 200 of CMC
    -   Quote symbol must be one of BTC, ETH, USDT, USD
    -   Listed at more than 3 exchanges(inclusive)

## Run

Crawl realtime trades:

```bash
docker run -it --rm -v $(pwd):/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -e REDIS_URL="redis://host-ip:6379" -e PARSER=true -u "$(id -u):$(id -g)" soulmachine/carbonbot:debian pm2-runtime start pm2.trade.config.js
```

The container above will deliver data to the following locations:

| Local           | AWS S3            | Redis Channel   |
| --------------- | ----------------- | --------------- |
| $DATA_DIR/trade | $AWS_S3_DIR/trade | carbonbot:trade |

Crawl funding rates:

```bash
docker run -it --rm -v $(pwd):/data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -e REDIS_URL="redis://host-ip:6379" -e PARSER=true -u "$(id -u):$(id -g)" soulmachine/carbonbot:debian pm2-runtime start pm2.funding_rate.config.js
```

The container above will deliver data to the following locations:

| Local                  | AWS S3                   | Redis Channel          |
| ---------------------- | ------------------------ | ---------------------- |
| $DATA_DIR/funding_rate | $AWS_S3_DIR/funding_rate | carbonbot:funding_rate |

## Build

```bash
docker build --squash -t soulmachine/carbonbot:debian -f Dockerfile.debian .
docker push soulmachine/carbonbot:debian

docker build --squash -t soulmachine/carbonbot:amazonlinux -f Dockerfile.amazonlinux .
docker push soulmachine/carbonbot:amazonlinux
```
