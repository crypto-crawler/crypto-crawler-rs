FROM rust:latest AS builder

RUN mkdir /project
WORKDIR /project
COPY ./ ./
RUN cargo build --release


FROM keymetrics/pm2:latest-slim

RUN mkdir /data
WORKDIR /data

COPY --from=builder /project/target/release/carbonbot /usr/local/bin/
COPY --from=builder /project/pm2.json /root/pm2.json

RUN pm2 install pm2-server-monit

ENV RUST_LOG "warn"
ENV DATA_DIR "/data"

VOLUME [ "/data" ]
EXPOSE 9615

# CMD ["carbonbot"]
CMD [ "pm2-runtime", "start", "/root/pm2.json", "--web" ]
