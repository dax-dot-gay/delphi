#! /bin/bash

CERTD=/workspaces/delphi/certs

COUNTRY=US
STATE=NewYork
LOCATION=NewYork

if [ ! -d "$CERTD" ]; then
    mkdir -p "$CERTD/client" "$CERTD/api"
    cd "$CERTD/client"
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 3650 -nodes -subj "/C=$COUNTRY/ST=$STATE/L=$LOCATION/CN=delphi.client"
    cd "$CERTD/api"
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 3650 -nodes -subj "/C=$COUNTRY/ST=$STATE/L=$LOCATION/CN=delphi.api"
    cd /workspaces/delphi
fi