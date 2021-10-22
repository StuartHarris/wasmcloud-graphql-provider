# `wasmcloud-graphql-provider`

Hosts [PostGraphile](https://graphile.org) as a [wasmCloud](https://wasmcloud.dev) 0.50 capability provider, exposing a postgres database as a GraphQL API for actors to consume.

## Todo

- [x] statically linked nodejs to host PostGraphile
- [x] neon bindings to call into PostGraphile middleware
- [x] correlate queries and responses
- [x] Smithy interface
- [x] wasmCloud provider
- [x] package node modules in binary
- [x] pass DATABASE_URL from link through to config
- [ ] pass through example actor to demo functionality
- [ ] tests

## Notes

1. This is a work in progress and not suitable for use anywhere, yet.
2. create a `provider/.env` file with the postgres `DATABASE_URL` (this will ultimately be configured by the wasmCloud link)
