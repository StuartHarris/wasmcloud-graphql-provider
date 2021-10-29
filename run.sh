#!/bin/bash
set -eux -o pipefail

ACTOR_ID=MA5PVZ6QNJK5TELQHPQGICJJ2EFVH7YDVXKF2NCUTYGSVVHUCEOL5UW6
ACTOR_REF=localhost:5000/v2/pass_through:0.1.0
HTTPSERVER_CONFIG=http_config.json
HTTPSERVER_ID=VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
HTTPSERVER_REF=wasmcloud.azurecr.io/httpserver:0.14.4
PROVIDER_CONTRACT=stuart-harris:graphql-provider
PROVIDER_ID=VAH3FDYDTRSPMDDHSO4TK6YOKXHZLFQ5QIT4TM4USZ4GKBU2BTJ2JIP5
PROVIDER_REF=localhost:5000/wasmcloud-graphql-provider:0.1.0

# command to base64 encode stdin to stdout
BASE64_ENC=base64

# base-64 encode file into a string
b64_encode_file() {
	$BASE64_ENC <"$1" | tr -d ' \r\n'
}

# start wasmcloud capability providers
# idempotent
start_providers() {
	wash ctl start actor $ACTOR_REF --timeout 30
	wash ctl start provider $HTTPSERVER_REF --link-name default --timeout 30
	wash ctl start provider $PROVIDER_REF --link-name default --timeout 30
}

# link actors with providers
# idempotent
link_providers() {
	wash ctl link put $ACTOR_ID $HTTPSERVER_ID \
		wasmcloud:httpserver config_b64="$(b64_encode_file $HTTPSERVER_CONFIG)"
	wash ctl link put $ACTOR_ID $PROVIDER_ID \
		$PROVIDER_CONTRACT "$(cat .env)"
}

start_providers
link_providers
