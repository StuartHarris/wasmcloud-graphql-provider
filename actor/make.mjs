#!/usr/bin/env zx

const config = {
  claims: [
    "stuart-harris:graphql-provider",
    "wasmcloud:builtin:logging",
    "wasmcloud:httpserver",
  ],
  registry: "registry:5001",
};

process.env.CARGO_TERM_COLOR = "always";
process.env.FORCE_COLOR = "3";

$.verbose = false;
const meta = JSON.parse(await $`cargo metadata --no-deps --format-version 1`);
$.verbose = true;
const project = meta.packages[0].name;
const version = meta.packages[0].version;
const revision = 0;
const build = argv.debug ? "debug" : "release";

if (argv.clean) {
  step("Cleaning...");
  await $`cargo clean`;
  await $`rm -rf build`;
}

if (argv.build) {
  step("Building...");
  await $`cargo build ${build === "release" ? "--release" : ""}`;
}

const unsigned_wasm = `target/wasm32-unknown-unknown/${build}/${project}.wasm`;
const signed_wasm = `build/${project}_s.wasm`;

if (argv.package) {
  step("Packaging...");
  await $`mkdir -p build`;

  await $`wash claims sign ${unsigned_wasm} ${[
    ...config.claims.flatMap((c) => ["--cap", c]),
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
  step("Pushing...");
  await $`wash reg push --insecure ${config.registry}/${project}:${version} ${signed_wasm}`;
}

function step(msg) {
  console.log(chalk.blue.bold(`\n${msg}`));
}
