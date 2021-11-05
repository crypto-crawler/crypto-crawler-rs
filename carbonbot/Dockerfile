FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim AS builder

RUN mkdir /project
WORKDIR /project

COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/

RUN apt -qy update && apt -qy install pkg-config libssl-dev \
 && RUSTFLAGS="-C target-cpu=x86-64-v3" cargo build --release


FROM node:bullseye-slim

COPY --from=builder /project/target/release/carbonbot /usr/local/bin/carbonbot

# procps provides the ps command, which is needed by pm2
RUN apt-get -qy update && apt-get -qy --no-install-recommends install \
    ca-certificates curl logrotate procps pigz \
 && chown -R node:node /var/lib/logrotate/ \
 && npm install pm2 -g --production \
 && apt-get -qy install gzip unzip && curl https://rclone.org/install.sh | bash \
 && apt-get -qy autoremove && apt-get clean && rm -rf /var/lib/apt/lists/* && rm -rf /tmp/*

# Install fixuid
RUN ARCH="$(dpkg --print-architecture)" && \
    curl -SsL https://github.com/boxboat/fixuid/releases/download/v0.5.1/fixuid-0.5.1-linux-amd64.tar.gz | tar -C /usr/local/bin -xzf - && \
    chown root:root /usr/local/bin/fixuid && \
    chmod 4755 /usr/local/bin/fixuid && \
    mkdir -p /etc/fixuid && \
    printf "user: node\ngroup: node\npaths:\n  - /home/node\n  - /var/lib/logrotate/\n" > /etc/fixuid/config.yml

COPY --chown=node:node ./conf/pm2/pm2.bbo.config.js /home/node/pm2.bbo.config.js
COPY --chown=node:node ./conf/pm2/pm2.candlestick.config.js /home/node/pm2.candlestick.config.js
COPY --chown=node:node ./conf/pm2/pm2.trade.config.js /home/node/pm2.trade.config.js
COPY --chown=node:node ./conf/pm2/pm2.ticker.config.js /home/node/pm2.ticker.config.js
COPY --chown=node:node ./conf/pm2/pm2.l2_event.config.js /home/node/pm2.l2_event.config.js
COPY --chown=node:node ./conf/pm2/pm2.l2_snapshot.config.js /home/node/pm2.l2_snapshot.config.js
COPY --chown=node:node ./conf/pm2/pm2.l2_topk.config.js /home/node/pm2.l2_topk.config.js
COPY --chown=node:node ./conf/pm2/pm2.l3_event.config.js /home/node/pm2.l3_event.config.js
COPY --chown=node:node ./conf/pm2/pm2.funding_rate.config.js /home/node/pm2.funding_rate.config.js
COPY --chown=node:node ./conf/pm2/pm2.other.config.js /home/node/pm2.other.config.js
COPY --chown=node:node ./conf/pm2/pm2.open_interest.config.js /home/node/pm2.open_interest.config.js

COPY ./conf/logrotate/logrotate.bbo.conf /usr/local/etc/logrotate.bbo.conf
COPY ./conf/logrotate/logrotate.candlestick.conf /usr/local/etc/logrotate.candlestick.conf
COPY ./conf/logrotate/logrotate.trade.conf /usr/local/etc/logrotate.trade.conf
COPY ./conf/logrotate/logrotate.ticker.conf /usr/local/etc/logrotate.ticker.conf
COPY ./conf/logrotate/logrotate.l2_event.conf /usr/local/etc/logrotate.l2_event.conf
COPY ./conf/logrotate/logrotate.l2_snapshot.conf /usr/local/etc/logrotate.l2_snapshot.conf
COPY ./conf/logrotate/logrotate.l2_topk.conf /usr/local/etc/logrotate.l2_topk.conf
COPY ./conf/logrotate/logrotate.l3_event.conf /usr/local/etc/logrotate.l3_event.conf
COPY ./conf/logrotate/logrotate.funding_rate.conf /usr/local/etc/logrotate.funding_rate.conf
COPY ./conf/logrotate/logrotate.other.conf /usr/local/etc/logrotate.other.conf
COPY ./conf/logrotate/logrotate.open_interest.conf /usr/local/etc/logrotate.open_interest.conf

COPY --chown=node:node ./conf/rclone.conf /home/node/.config/rclone/rclone.conf
COPY ./conf/upload.sh /usr/local/bin/upload.sh
COPY ./conf/logrotate.sh /usr/local/bin/logrotate.sh

ENV RUST_LOG "warn"
ENV RUST_BACKTRACE 1

VOLUME [ "/data" ]

USER node:node
ENV USER node
WORKDIR /home/node

ENTRYPOINT ["fixuid", "-q"]
