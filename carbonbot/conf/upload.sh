#!/bin/bash

# This file is optional and it acts as a garbage collector which uploads files to S3.
# These files should be uploaded by logrotate but somehow they are not.

market_type=$1

# Infinite while loop
while :
do
  sleep 60
  # Find files older than 15 minutes
  find $DATA_DIR/$market_type -name "*.json" -type f -mmin +15 | xargs -r pigz
  rclone move $DATA_DIR/$market_type $AWS_S3_DIR/$market_type --include '*.json.gz' --no-traverse --min-age 15m
done
