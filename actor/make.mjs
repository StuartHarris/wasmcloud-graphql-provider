#!/usr/bin/env zx

import { getProject, ifChanged, setColors, step } from "../automation/lib.mjs";

const config = {
  claims: [
    "stuart-harris:graphql-provider",
    "wasmcloud:builtin:logging",
    "wasmcloud:httpserver",
  ],
  registry: "registry:5001",
};

setColors();

const { name: project, version } = await getProject();
const revision = 0;
const build = argv.debug ? "debug" : "release";

if (argv.clean) {
  step("Cleaning...");
  await $`cargo clean`;
  await $`rm -rf build`;
}

const destination = `build/${project}_s.wasm`;

if (argv.build) {
  step("Building...");
  await ifChanged(".", "build", async () => {
    await $`cargo build ${build === "release" ? "--release" : ""}`;
    await fs.ensureDir("build");

    const source = `target/wasm32-unknown-unknown/${build}/${project}.wasm`;
    await $`wash claims sign ${source} ${[
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
  });

  await $`wash claims inspect ${destination}`;
}

if (argv.push) {
  step("Pushing...");
  await $`wash reg push --insecure ${config.registry}/${project}:${version} ${destination}`;
}
