# CarbonBot

## Which trading pairs/symbols are chosen?

- All pairs/symbols in contract markets, including futures, perpetual swap and options.
- For pairs/symbols in spot market, all of the following criteria must be met:

    - Base symbol must be in top 200 of CMC
    - Quote symbol must be one of BTC, ETH, USDT, USD
    - Listed at more than 3 exchanges(inclusive)

## Run

```bash
docker run -it --rm -v $(pwd):/data soulmachine/carbonbot:debian pm2-runtime start pm2.trade.json
```

To enable crawling mxc spot markets, we need to define a `MXC_ACCESS_KEY` environment variable(because this cryprocurrency exchange requires it even for public APIs, while other exchanges don't):

```bash
docker run -it --rm -v $(pwd):/data -e MXC_ACCESS_KEY=your_mxc_access_key soulmachine/carbonbot:debian pm2-runtime start pm2.trade.json
```

## Build

```bash
docker build -t soulmachine/carbonbot:debian -f Dockerfile.debian .
docker push soulmachine/carbonbot:debian
```
