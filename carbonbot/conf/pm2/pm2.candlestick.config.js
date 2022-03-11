// see src/market_type.rs in crypto-markets
const market_types = {
    binance: [
        "spot",
        "linear_future",
        "inverse_future",
        "linear_swap",
        "inverse_swap",
    ],
    bitfinex: ["spot", "linear_swap"],
    bitget: ["inverse_swap", "linear_swap"],
    bitmex: ["unknown"], // unknown means all
    bybit: ["inverse_future", "inverse_swap", "linear_swap"],
    deribit: ["inverse_future", "inverse_swap", "european_option"],
    gate: ["spot", "linear_future", "linear_swap", "inverse_swap"],
    huobi: ["spot", "inverse_future", "linear_swap", "inverse_swap"],
    kraken: ["spot"],
    kucoin: ["spot", "inverse_future", "linear_swap", "inverse_swap"],
    mexc: ["spot", "linear_swap", "inverse_swap"],
    okx: [
        "spot",
        "linear_future",
        "inverse_future",
        "linear_swap",
        "inverse_swap",
        "european_option",
    ],
    zbg: ["spot", "inverse_swap", "linear_swap"],
};

const apps = [];

Object.keys(market_types).forEach((exchange) => {
    market_types[exchange].forEach((market_ype) => {
        const app = {
            name: `crawler-candlestick-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} candlestick`,
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
    script: "/usr/local/bin/logrotate.sh",
    args: "/usr/local/etc/logrotate.candlestick.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

apps.push({
    name: "upload",
    script: "/usr/local/bin/upload.sh",
    args: "candlestick",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

module.exports = {
    apps,
};
