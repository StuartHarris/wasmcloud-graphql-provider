# `wasmcloud-graphql-provider`

Hosts [PostGraphile](https://graphile.org) as a [wasmCloud](https://wasmcloud.dev) 0.50 capability provider, exposing a postgres database as a GraphQL API for actors to consume.

![architecture](docs/GraphQL%20Provider.svg)

## Notes

This is a work in progress and not suitable for use anywhere, yet. :-)

The pass-through actor is for demo purposes only — you might instead just consume the GraphQL in your actors, rather than exposing it directly to a browser client.

There may be some an outstanding problem that surfaces when large payloads (like an introspection query) are requested (in addition to the default 1MB limit on NATS).

There is a dev container configuration in the project to build the provider for linux (e.g. if you're on a Mac).

## Todo

- [x] statically linked nodejs to host PostGraphile
- [x] neon bindings to call into PostGraphile middleware
- [x] correlate queries and responses
- [x] Smithy interface
- [x] wasmCloud provider
- [x] package node modules in binary
- [x] pass DATABASE_URL from link through to config
- [x] pass through example actor to demo functionality
- [x] don't block async in handler
- [x] pass HTTP headers through
