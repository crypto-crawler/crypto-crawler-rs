// see src/market_type.rs in crypto-markets
const market_types = {
    binance: ["spot", "linear_swap", "inverse_swap"],
    bitfinex: ["spot", "linear_swap"],
    bitget: ["inverse_swap", "linear_swap"],
    bithumb: ["spot"],
    bitz: ["spot"],
    bybit: ["inverse_future", "inverse_swap", "linear_swap"],
    coinbase_pro: ["spot"],
    deribit: ["inverse_future", "option"], // inverse_swap is included in inverse_future
    gate: ["spot", "linear_future", "linear_swap", "inverse_swap"],
    huobi: ["spot", "inverse_future", "linear_swap", "inverse_swap", "option"],
    kraken: ["spot"],
    kucoin: ["spot", "inverse_future", "linear_swap", "inverse_swap"],
    mxc: ["linear_swap", "inverse_swap"],
    okex: [
        "spot",
        "linear_future",
        "inverse_future",
        "linear_swap",
        "inverse_swap",
        "option",
    ],
    zbg: ["spot", "inverse_swap", "linear_swap"],
};

const apps = [];

Object.keys(market_types).forEach((exchange) => {
    market_types[exchange].forEach((market_ype) => {
        const app = {
            name: `crawler-ticker-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} ticker`,
            exec_interpreter: "none",
            exec_mode: "fork_mode",
            instances: 1,
            restart_delay: 5000, // 5 seconds
        };

        apps.push(app);
    });
});

apps.push({
    name: "logrotate",
    script: "logrotate",
    args: "/usr/local/etc/logrotate.ticker.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

module.exports = {
    apps,
};
