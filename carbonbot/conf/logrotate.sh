#!/bin/bash
# Linted by https://www.shellcheck.net/

# This is a thin wrapper around logrotate, to make it run only
# at "*/15 * * * *"

minute=$(date +%M)

if [ $(( minute % 15 )) == 0 ]; then
    logrotate "$@"
else
    echo "Current timestamp is not 15 minutes, skipped"
    exit 0
fi
