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

const { name: project, version } = await getProject();
const revision = 0;
const build = argv.debug ? "debug" : "release";

if (argv.clean) {
  step("Cleaning...");
  await $`cargo clean`;
  await $`rm -rf build node_modules`;
}

if (argv.build) {
  step("Building...");
  await ifChanged(".", "./build", async () => {
    await $`mkdir -p ./build/dist`;
    await $`yarn`;
    await $`yarn build`;
    await $`yarn --production`;
    await $`tar -czf ./build/build.tgz node_modules -C build dist`;
    await $`yarn`; // re-add dev deps for next edit
    await $`cargo build ${build === "release" ? "--release" : ""}`;
  });
}

const source = `target/${build}/${project}`;
const destination = `build/${project}.par.gz`;

if (argv.package) {
  step("Packaging...");
  await $`mkdir -p build`;

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
  await $`wash par inspect ${destination}`;
}

if (argv.push) {
  step("Pushing...");
  await $`wash reg push --insecure ${config.registry}/${project}:${version} ${destination}`;
}
