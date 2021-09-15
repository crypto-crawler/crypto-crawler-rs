#!/bin/bash
# Linted by https://www.shellcheck.net/

# Inside Docker logrotate always run at first time, which is not expected.
# This script is a thin wrapper around logrotate, to make it skip the first
# run if timestamp is not at "*/15 * * * *"

# https://unix.stackexchange.com/a/79372/40515
minute=$(date +%-M)

if [ ! -f /tmp/logrotate.first.done ] ; then
    if [ $(( minute % 15 )) != 0 ]; then
      echo "Fist time run and timestamp is not 15 minutes, skipped"
    else
      logrotate "$@"
    fi
    touch /tmp/logrotate.first.done
else
    logrotate "$@"
fi
