#!/bin/bash
set -xeu

if [ ! -f /opt/subservices/data/setup-complete ]; then
    /app/substreams-sink-sql setup $DSN /opt/subservices/config/substreams.spkg --postgraphile && touch /opt/subservices/data/setup-complete
fi

/app/substreams-sink-sql run $DSN /opt/subservices/config/substreams.spkg --on-module-hash-mistmatch=warn
