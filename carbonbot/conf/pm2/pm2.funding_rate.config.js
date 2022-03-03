// see src/market_type.rs in crypto-markets
const market_types = {
    binance: ["linear_swap", "inverse_swap"],
    bitget: ["inverse_swap", "linear_swap"],
    bitmex: ["unknown"], // unknown means all
    huobi: ["linear_swap", "inverse_swap"],
    okx: ["linear_swap", "inverse_swap"],
};

const apps = [];

Object.keys(market_types).forEach((exchange) => {
    market_types[exchange].forEach((market_ype) => {
        const app = {
            name: `crawler-funding-rate-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} funding_rate`,
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
    args: "/usr/local/etc/logrotate.funding_rate.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

apps.push({
    name: "upload",
    script: "/usr/local/bin/upload.sh",
    args: "funding_rate",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

module.exports = {
    apps,
};
