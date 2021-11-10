#!/usr/bin/env zx

import {
  getArch,
  getProject,
  ifChanged,
  setColors,
  step,
} from "../automation/lib.mjs";

const config = {
  capability: "stuart-harris:graphql-provider",
  vendor: "StuartHarris",
  registry: "registry:5001",
};

setColors();

const { name: project, version } = await getProject("rust");
const revision = 0;
const build = argv.debug ? "debug" : "release";

if (argv.clean) {
  await $`rm -rf build`;

  step("Cleaning node ...");
  cd("node");
  await $`rm -rf build dist node_modules`;

  step("Cleaning rust ...");
  cd("rust");
  await $`cargo clean`;
  await $`rm -rf build`;
}

const destination = `build/${project}.par.gz`;

if (argv.build) {
  await fs.ensureDir("build");
  await fs.ensureDir("node/dist");
  await fs.ensureDir("node/build");
  await fs.ensureDir("rust/build");

  step("Building node...");
  cd("node");
  await ifChanged(".", "build", async () => {
    await $`yarn`;
    await $`yarn build`;
    await $`yarn --production`;
    await $`tar -czf ./build/build.tgz node_modules -C dist .`;
    await $`yarn`; // re-add dev deps for next edit
  });

  step("Building rust...");
  cd("rust");
  await ifChanged([".", "../node"], "build", async () => {
    await $`cargo build ${build === "release" ? "--release" : ""}`;

    cd(".");
    const source = `rust/target/${build}/${project}`;
    await $`wash par create ${[
      "--arch",
      getArch(),
      "--binary",
      source,
      "--capid",
      config.capability,
      "--name",
      project,
      "--vendor",
      config.vendor,
      "--version",
      version,
      "--revision",
      revision,
      "--destination",
      destination,
      "--compress",
    ]}`;
  });

  cd(".");
  await $`wash par inspect ${destination}`;
}

if (argv.push) {
  step("Pushing...");
  cd(".");
  await $`wash reg push --insecure ${config.registry}/${project}:${version} ${destination}`;
}
