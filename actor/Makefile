# Makefile for pass-through

PROJECT  = pass_through
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:httpserver stuart-harris:graphql-provider wasmcloud:builtin:logging
# registry url for our actor
REG_URL  = registry:5001/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

# friendly name for the actor
ACTOR_NAME = "pass-through"
# optional call alias for actor
# ACTOR_ALIAS=nickname

include ./actor.mk
