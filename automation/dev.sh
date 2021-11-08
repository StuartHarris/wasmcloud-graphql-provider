#!/bin/bash

set -a
# shellcheck disable=SC1091
source ../.env
set +a

postgraphile \
	--allow-explain \
	--append-plugins @graphile-contrib/pg-many-to-many \
	--append-plugins @graphile-contrib/pg-simplify-inflector \
	--append-plugins postgraphile-plugin-connection-filter \
	--connection "$DATABASE_URL" \
	--dynamic-json \
	--enable-query-batching \
	--enhance-graphiql \
	--export-schema-graphql schema.graphql \
	--extended-errors hint,detail,errcode \
	--graphiql "/" \
	--legacy-relations omit \
	--no-ignore-indexes \
	--no-ignore-rbac \
	--no-setof-functions-contain-nulls \
	--schema public \
	--show-error-stack=json \
	--watch
