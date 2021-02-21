# CarbonBot

## Which trading pairs/symbols are chosen?

- All pairs/symbols in contract markets, including futures, perpetual swap and options.
- For pairs/symbols in spot market, all of the following criteria must be met:

    - Base symbol must be in top 200 of CMC
    - Quote symbol must be one of BTC, ETH, USDT, USD
    - Listed at more than 3 exchanges(inclusive)

## Run

```bash
docker run -it --rm -v $(pwd):/data -e AWS_ACCESS_KEY_ID=AKIAJDDG3LQ4XSBLKFJQ -e AWS_SECRET_ACCESS_KEY="29W4ruPHL+PtFtjNaDmClP8s4mBqzbmkbtKSr7Bx" -e AWS_S3_DIR="s3://your_path" soulmachine/carbonbot:debian pm2-runtime start pm2.trade.config.js

```

## Build

```bash
docker build -t soulmachine/carbonbot:debian -f Dockerfile.debian .
docker push soulmachine/carbonbot:debian

docker build -t soulmachine/carbonbot:amazonlinux -f Dockerfile.amazonlinux .
docker push soulmachine/carbonbot:amazonlinux
```
