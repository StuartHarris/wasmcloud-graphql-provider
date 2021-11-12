# `wasmcloud-graphql-provider`

Hosts [PostGraphile](https://graphile.org) as a [wasmCloud](https://wasmcloud.dev) 0.50 capability provider, exposing a postgres database as a GraphQL API for actors to consume.

![architecture](docs/GraphQL%20Provider.svg)

## Getting started

The default local setup is to run NATS, the registry, and the postgres database in Docker using the `automation/docker-compose.yaml` file, and to run `wasmcloud` natively.

1. build and sign everything. If you're on MacOS, and you need to build the provider for Linux, you can run the provider's `make` in the dev container — for vscode, run (`cmd-shift-P`) `@command:remote-containers.reopenInContainer`.

   ```sh
   make

   # or
   (cd interface && make)
   (cd actor && make)
   (cd provider && make)
   ```

2. run up a system (postgres DB, pgAdmin, OCI registry, NATS, wasmCloud)

   ```sh
   # install `sqlx-cli` to create db and run migrations
   cargo install sqlx-cli

   # install `zx` to run scripts
   npm install -g zx

   # install `dirsh` to check if inputs have changed for build optimisation
   cargo install dirsh

   # optional: install `monobuild` to support building all changed dependencies in CI pipelines
   cargo install --git https://github.com/charypar/monobuild

   # create a .env file with the database url, e.g.
   echo `DATABASE_URL=postgresql://postgres:changeme@localhost:5432/todos` >.env

   cd automation

   # bring up the docker containers and wasmcloud
   ./run.mjs --up

   # bring up the actor, providers and links
   ./run.mjs --start

   # tail the logs — you should see "postgraphile initializing at postgresql:****@localhost:5432/todos"
   tail -f  ~/wasmcloud/var/log/erlang.log.1

   # test
   curl -vv -H 'Content-Type: application/json' -d@query.json localhost:8080

   # bring everything down
   ./run.mjs --down
   ```

3. Some example GraphQL queries:

   ```graphql
   query get_all {
     todos {
       totalCount
       edges {
         cursor
         node {
           id
           nodeId
           content
           createdAt
           updatedAt
           completedAt
         }
       }
     }
   }

   mutation create_one {
     createTodo(input: { todo: { content: "test" } }) {
       todo {
         id
         nodeId
         content
         createdAt
         updatedAt
         completedAt
       }
     }
   }

   mutation delete_one {
     deleteTodo(input: { id: 1 }) {
       todo {
         id
       }
     }
   }
   ```

## Notes

This is a work in progress and not suitable for use anywhere, yet. :-)

The pass-through actor is really just to demo functionality — you might instead just consume the GraphQL in your actors, rather than exposing it directly to a browser client.

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
