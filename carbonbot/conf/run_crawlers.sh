#!/bin/bash
LOCAL_DATA_DIR="/your/local/path"       # required
AWS_ACCESS_KEY_ID="you access key"      # required
AWS_SECRET_ACCESS_KEY="your secret key" # required
AWS_S3_DIR="s3://bucket/your/s3/path"   # required

docker pull soulmachine/carbonbot

docker stop $(docker ps -aq --filter "name=carbonbot")
docker rm $(docker ps -aq --filter "name=carbonbot")

# l2_snapshot and open_interest are not included, better deploy them in a different network
msg_types=("trade" "l2_event" "l2_topk" "l3_event" "bbo" "ticker" "candlestick" "funding_rate" "other")

for msg_type in ${msg_types[@]}; do
  docker run -d --name carbonbot-$msg_type --restart always -v $LOCAL_DATA_DIR/data -e DATA_DIR=/data -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY -e AWS_S3_DIR=$AWS_S3_DIR -u "$(id -u):$(id -g)" soulmachine/carbonbot pm2-runtime start pm2.$msg_type.config.js
done

docker system prune -af
