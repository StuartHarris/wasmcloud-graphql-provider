#!/usr/bin/env zx

$.verbose = false;

const claims = [
  "stuart-harris:graphql-provider",
  "wasmcloud:builtin:logging",
  "wasmcloud:httpserver",
];
const registry = "registry:5001";

const meta = JSON.parse(await $`cargo metadata --no-deps --format-version 1`);
const project = meta.packages[0].name;
const version = meta.packages[0].version;
const revision = 0;
const build = argv.debug ? "debug" : "release";

$.verbose = true;

if (argv.clean) {
  console.log(chalk.blueBright.bold("Cleaning..."));
  await $`cargo clean`;
  await $`rm -rf build`;
}

if (argv.build) {
  console.log(chalk.blueBright.bold("Building..."));
  await $`cargo build ${build === "release" ? "--release" : ""}`;
}

const unsigned_wasm = `target/wasm32-unknown-unknown/${build}/${project}.wasm`;
const signed_wasm = `build/${project}_s.wasm`;

if (argv.package) {
  console.log(chalk.blueBright.bold("Packaging..."));
  await $`mkdir -p build`;

  await $`wash claims sign ${unsigned_wasm} ${[
    ...claims.flatMap((c) => ["--cap", c]),
    "--name",
    project,
    "--ver",
    version,
    "--rev",
    revision,
    "--destination",
    `build/${project}_s.wasm`,
  ]}`;
  await $`wash claims inspect ${signed_wasm}`;
}

if (argv.push) {
  console.log(chalk.blueBright.bold("Pushing..."));
  await $`wash reg push --insecure ${registry}/${project}:${version} ${signed_wasm}`;
}
