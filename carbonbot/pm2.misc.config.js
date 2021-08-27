const apps = [];

apps.push({
    name: "crawler-bitmex-misc",
    script: "bitmex_misc",
    exec_interpreter: "none",
    exec_mode: "fork",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

apps.push({
    name: "crawler-huobi-misc-spot",
    script: "huobi_misc spot",
    exec_interpreter: "none",
    exec_mode: "fork",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});
apps.push({
    name: "crawler-huobi-misc-inverse-future",
    script: "huobi_misc inverse_future",
    exec_interpreter: "none",
    exec_mode: "fork",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});
apps.push({
    name: "crawler-huobi-misc-inverse-swap",
    script: "huobi_misc inverse_swap",
    exec_interpreter: "none",
    exec_mode: "fork",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});
apps.push({
    name: "crawler-huobi-misc-linear-swap",
    script: "huobi_misc linear_swap",
    exec_interpreter: "none",
    exec_mode: "fork",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});
apps.push({
    name: "crawler-huobi-misc-option",
    script: "huobi_misc european_option",
    exec_interpreter: "none",
    exec_mode: "fork",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

apps.push({
    name: "logrotate",
    script: "logrotate",
    args: "/usr/local/etc/logrotate.misc.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

apps.push({
    name: "upload",
    script: "upload.sh",
    args: "misc",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

module.exports = {
    apps,
};
