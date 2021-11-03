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
  config: (await $`cat .env`).stdout.replace("\n", ""),
};
const HTTPSERVER = {
  id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M",
  ref: "wasmcloud.azurecr.io/httpserver:0.14.5",
  contract: "wasmcloud:httpserver",
  config: `config_b64=${btoa(JSON.stringify({ address: "0.0.0.0:8080" }))}`,
};

async function start() {
  await $`wash ctl start actor ${ACTOR.ref} --timeout 30`;
  await $`wash ctl start provider ${HTTPSERVER.ref} --link-name default --timeout 30`;
  await $`wash ctl start provider ${PROVIDER.ref} --link-name default --timeout 30`;
}

async function links() {
  await $`wash ctl link put ${ACTOR.id} ${HTTPSERVER.id} ${HTTPSERVER.contract} ${HTTPSERVER.config}`;
  await $`wash ctl link put ${ACTOR.id} ${PROVIDER.id} ${PROVIDER.contract} ${PROVIDER.config}`;
}

await start();
await links();
