#!/bin/bash
if test -z "$AWS_ACCESS_KEY_ID"
then
    >&2 echo "AWS_ACCESS_KEY_ID is empty"
    exit 1
fi

if test -z "$AWS_SECRET_ACCESS_KEY"
then
    >&2 echo "AWS_SECRET_ACCESS_KEY is empty"
    exit 1
fi

if test -z "$AWS_S3_DIR"
then
    >&2 echo "AWS_S3_DIR is empty"
    exit 1
fi

if test -z "$DATA_DIR"
then
    >&2 echo "DATA_DIR is empty"
    exit 1
fi

# Delete .gz files older than 3 minutes and smaller than 32 bytes
find $DATA_DIR -type f -name '*.json.gz' -mmin +3 -size -32c | xargs rm

# compress json files older than 61 minutes and larger than 64 bytes
find $DATA_DIR -type f -name '*.json' -mmin +61 -size +64c | xargs -r gzip

# move from local to AWS S3
rclone move $DATA_DIR $AWS_S3_DIR --min-age 1m --min-size 64B --include '*.json.gz' --no-traverse
