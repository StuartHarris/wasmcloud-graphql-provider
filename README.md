# `wasmcloud-graphql-provider`

Hosts [PostGraphile](https://graphile.org) as a [wasmCloud](https://wasmcloud.dev) 0.50 capability provider, exposing a postgres database as a GraphQL API for actors to consume.

![architecture](docs/GraphQL%20Provider.svg)

## Notes

The pass-through actor is for demo purposes only — you might instead just consume the GraphQL in your actors, rather than exposing it directly to a browser client.

There are some outstanding problems. I haven't built it for wasmCloud hosted in Docker, as that uses Debian Buster, and I think we need Bullseye for the required glibc version (>=2.9). It currently works on wasmCloud hosted on MacOS, but there are problems that surface when large payloads (like an introspection query) are requested (in addition to the default 1MB limit on NATS).

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

## Notes

This is a work in progress and not suitable for use anywhere, yet. :-)
