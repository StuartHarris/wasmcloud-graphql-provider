#!/usr/bin/env zx

import { retry, setColors, step } from "./lib.mjs";

setColors();

const REGISTRY = "registry:5001";
const ACTOR = {
  id: "MA5PVZ6QNJK5TELQHPQGICJJ2EFVH7YDVXKF2NCUTYGSVVHUCEOL5UW6",
  ref: `${REGISTRY}/pass_through:0.1.0`,
};
const PROVIDER = {
  id: "VAH3FDYDTRSPMDDHSO4TK6YOKXHZLFQ5QIT4TM4USZ4GKBU2BTJ2JIP5",
  ref: `${REGISTRY}/wasmcloud-graphql-provider:0.1.0`,
  contract: "stuart-harris:graphql-provider",
  config: (await fs.readFile("../.env")).toString().replace("\n", ""),
};
const HTTPSERVER = {
  id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M",
  ref: "wasmcloud.azurecr.io/httpserver:0.14.6",
  contract: "wasmcloud:httpserver",
  config: `config_b64=${btoa(JSON.stringify({ address: "0.0.0.0:8080" }))}`,
};

if (argv.up) {
  step("Starting containers");
  await $`docker compose up -d`;
  await sleep(1000);
  await retry({ count: 10, delay: 5000 }, async () => {
    await $`sqlx database create`;
    await $`sqlx migrate run`;
  });
  await $`rm -f ~/wasmcloud/var/log/erlang.log.*`;
  await $`WASMCLOUD_OCI_ALLOWED_INSECURE=registry:5001 ~/wasmcloud/bin/wasmcloud_host start`;
}

if (argv.start) {
  step("starting workloads");
  await $`(cd ../actor && make push)`;
  await $`(cd ../provider && make push)`;
  await $`wash ctl start actor ${ACTOR.ref} --timeout 30`;

  await $`wash ctl link put ${ACTOR.id} ${HTTPSERVER.id} ${HTTPSERVER.contract} ${HTTPSERVER.config}`;
  await $`wash ctl start provider ${HTTPSERVER.ref} --link-name default --timeout 30`;

  await $`wash ctl link put ${ACTOR.id} ${PROVIDER.id} ${PROVIDER.contract} ${PROVIDER.config}`;
  await $`wash ctl start provider ${PROVIDER.ref} --link-name default --timeout 30`;
}

if (argv.restart_provider) {
  step("restarting provider");
  const host = await getHost();
  await $`wash ctl stop provider ${host} ${PROVIDER.id} default ${PROVIDER.contract} --timeout 30`;
  await $`wash drain all`;
  await $`wash ctl start provider ${PROVIDER.ref} --link-name default --timeout 30`;
}

if (argv.stop) {
  step("stop workloads");
  const host = await getHost();
  await $`wash ctl stop actor ${host} ${ACTOR.id} --timeout 30`;
  await $`wash ctl stop provider ${host} ${PROVIDER.id} default ${PROVIDER.contract} --timeout 30`;
  await $`wash ctl stop provider ${host} ${HTTPSERVER.id} default ${HTTPSERVER.contract} --timeout 30`;
  await $`wash ctl link del ${ACTOR.id} ${PROVIDER.contract}`;
  await $`wash ctl link del ${ACTOR.id} ${HTTPSERVER.contract}`;
}

if (argv.down) {
  step("stopping containers");
  await $`~/wasmcloud/bin/wasmcloud_host stop`;
  await $`docker compose down`;
  await $`pkill -f wasmcloudcache`;
}

async function getHost() {
  $.verbose = false;
  const host = JSON.parse(await $`wash ctl get hosts --output json`).hosts[0]
    .id;
  $.verbose = true;
  return host;
}
