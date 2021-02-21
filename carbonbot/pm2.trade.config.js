// see src/market_type.rs in crypto-markets
const market_types = {
    binance: ["spot", "inverse_future", "linear_swap", "inverse_swap"],
    bitfinex: ["spot", "linear_swap"],
    bitget: ["inverse_swap", "linear_swap"],
    bitmex: [
        "inverse_swap",
        "quanto_swap",
        "linear_future",
        "inverse_future",
        "quanto_future",
    ],
    bitstamp: ["spot"],
    bybit: ["inverse_swap", "linear_swap"],
    coinbase_pro: ["spot"],
    deribit: ["inverse_future", "inverse_swap", "option"],
    ftx: ["spot", "linear_swap", "linear_future", "move", "bvol"],
    huobi: ["spot", "inverse_future", "linear_swap", "inverse_swap", "option"],
    kraken: ["spot"],
    mxc: ["spot", "linear_swap", "inverse_swap"],
    okex: [
        "spot",
        "linear_future",
        "inverse_future",
        "linear_swap",
        "inverse_swap",
        "option",
    ],
};

const apps = [];

Object.keys(market_types).forEach((exchange) => {
    market_types[exchange].forEach((market_ype) => {
        const app = {
            name: `crawler-trade-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} trade`,
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
    args: "/usr/local/etc/logrotate.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

module.exports = {
    apps,
};
