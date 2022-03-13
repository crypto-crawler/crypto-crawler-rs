// see src/market_type.rs in crypto-markets
const market_types = {
    binance: [
        "spot",
        "linear_future",
        "inverse_future",
        "linear_swap",
        "inverse_swap",
    ],
    bitget: ["inverse_swap", "linear_swap"],
    bitmex: ["unknown"], // unknown means all
    bitstamp: ["spot"],
    deribit: ["inverse_future", "inverse_swap", "european_option"],
    huobi: ["spot", "inverse_future", "linear_swap", "inverse_swap"],
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
};

const apps = [];

Object.keys(market_types).forEach((exchange) => {
    market_types[exchange].forEach((market_ype) => {
        const app = {
            name: `crawler-l2_topk-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} l2_topk`,
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
    args: "/usr/local/etc/logrotate.l2_topk.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

apps.push({
    name: "upload",
    script: "/usr/local/bin/upload.sh",
    args: "l2_topk",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

module.exports = {
    apps,
};
