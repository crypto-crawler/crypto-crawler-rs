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
    bithumb: ["spot"],
    bitmex: [
        "inverse_swap",
        "quanto_swap",
        "linear_future",
        "inverse_future",
        "quanto_future",
    ],
    bitstamp: ["spot"],
    bitz: ["spot"],
    bybit: ["inverse_future", "inverse_swap", "linear_swap"],
    coinbase_pro: ["spot"],
    deribit: ["inverse_future", "european_option"], // inverse_swap is included in inverse_future
    ftx: ["spot", "linear_swap", "linear_future", "move", "bvol"],
    gate: ["spot", "linear_future", "linear_swap", "inverse_swap"],
    huobi: ["spot", "inverse_future", "linear_swap", "inverse_swap"],
    kraken: ["spot"],
    kucoin: ["spot", "inverse_future", "linear_swap", "inverse_swap"],
    mxc: ["spot", "linear_swap", "inverse_swap"],
    okex: [
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
            name: `crawler-l2_event-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} l2_event`,
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
    args: "/usr/local/etc/logrotate.l2_event.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

apps.push({
    name: "upload",
    script: "upload.sh",
    args: "l2_event",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

module.exports = {
    apps,
};
