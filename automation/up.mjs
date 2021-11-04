#!/usr/bin/env zx

const REGISTRY = "registry:5001";
const ACTOR = {
  id: "MA5PVZ6QNJK5TELQHPQGICJJ2EFVH7YDVXKF2NCUTYGSVVHUCEOL5UW6",
  ref: `${REGISTRY}/pass_through:0.1.0`,
};
const PROVIDER = {
  id: "VAH3FDYDTRSPMDDHSO4TK6YOKXHZLFQ5QIT4TM4USZ4GKBU2BTJ2JIP5",
  ref: `${REGISTRY}/wasmcloud-graphql-provider:0.1.0`,
  contract: "stuart-harris:graphql-provider",
  config: (await $`cat ../.env`).stdout
    .replace("\n", "")
    .replace("localhost", "host.docker.internal"),
};
const HTTPSERVER = {
  id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M",
  ref: "wasmcloud.azurecr.io/httpserver:0.14.5",
  contract: "wasmcloud:httpserver",
  config: `config_b64=${btoa(JSON.stringify({ address: "0.0.0.0:8080" }))}`,
};

async function sys_up() {
  await $`docker compose up -d`;
}

async function db_up() {
  await $`sqlx database create`;
  await $`sqlx migrate run`;
}

async function start() {
  await $`(cd ../actor && make push)`;
  await $`(cd ../provider && make push)`;
  await $`wash ctl start actor ${ACTOR.ref} --timeout 30`;
  await $`wash ctl start provider ${HTTPSERVER.ref} --link-name default --timeout 30`;
  await $`wash ctl start provider ${PROVIDER.ref} --link-name default --timeout 30`;
}

async function links() {
  await $`wash ctl link put ${ACTOR.id} ${HTTPSERVER.id} ${HTTPSERVER.contract} ${HTTPSERVER.config}`;
  await $`wash ctl link put ${ACTOR.id} ${PROVIDER.id} ${PROVIDER.contract} ${PROVIDER.config}`;
}

await sys_up();
await retry({ count: 10, delay: 5000 }, db_up);
await retry({ count: 10, delay: 5000 }, async () => {
  await start();
  await links();
});

async function retry(
  { count, delay = 5000 },
  f,
  evaluator = async () => false
) {
  for (let i = 0; i < count; i++) {
    try {
      return await f();
    } catch (e) {
      let done = await evaluator(e);
      if (done) return e;
      await new Promise((Y) => setTimeout(Y, delay));
    }
  }
}
