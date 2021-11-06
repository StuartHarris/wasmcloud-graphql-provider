#!/usr/bin/env zx

$.verbose = false;

const claims = [
  "stuart-harris:graphql-provider",
  "wasmcloud:builtin:logging",
  "wasmcloud:httpserver",
];
const meta = JSON.parse(await $`cargo metadata --no-deps --format-version 1`);
const project = meta.packages[0].name;
const version = meta.packages[0].version;
const build = argv.debug ? "debug" : "release";

const unsigned_wasm = `target/wasm32-unknown-unknown/${build}/${project}.wasm`;

if (argv.clean) {
  console.log(chalk.green("Clean..."));
  await $`cargo clean`;
  await $`rm -rf build`;
}

console.log(chalk.green("Build..."));
$.verbose = true;
await $`cargo build ${build === "release" ? "--release" : ""}`;
$.verbose = false;

console.log(chalk.green("Sign..."));
await $`mkdir -p build`;
await $`wash claims sign ${unsigned_wasm} ${[
  ...claims.flatMap((c) => ["--cap", c]),
  "--name",
  project,
  "--ver",
  version,
  "--destination",
  `build/${project}_s.wasm`,
]}`;
await $`wash claims inspect ${`build/${project}_s.wasm`}`.pipe(process.stdout);
