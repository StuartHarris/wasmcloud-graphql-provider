#!/usr/bin/env zx

$.verbose = false;

const capability = "stuart-harris:graphql-provider";
const registry = "registry:5001";
const vendor = "StuartHarris";

const meta = JSON.parse(await $`cargo metadata --no-deps --format-version 1`);
const project = meta.packages[0].name;
const version = meta.packages[0].version;
const revision = 0;
const build = argv.debug ? "debug" : "release";
const operating_systems = {
  darwin: "macos",
  linux: "linux",
};
const architectures = {
  x64: "x86_64",
};
const arch = `${architectures[os.arch()]}-${operating_systems[os.platform()]}`;

$.verbose = true;

if (argv.clean) {
  console.log(chalk.blueBright.bold("Cleaning..."));
  await $`cargo clean`;
  await $`rm -rf build dist`;
}

if (argv.build) {
  console.log(chalk.blueBright.bold("Building..."));
  await $`yarn`;
  await $`yarn build`;
  await $`yarn --production`;
  await $`mkdir -p build`;
  await $`tar -czf build/build.tgz node_modules dist`;
  await $`cargo build ${build === "release" ? "--release" : ""}`;
  await $`yarn`; // re-add dev deps for next edit
}

const source = `target/${build}/${project}`;
const destination = `build/${project}.par.gz`;

if (argv.package) {
  console.log(chalk.blueBright.bold("Packaging..."));
  await $`mkdir -p build`;

  await $`wash par create ${[
    "--arch",
    arch,
    "--binary",
    source,
    "--capid",
    capability,
    "--name",
    project,
    "--vendor",
    vendor,
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
  console.log(chalk.blueBright.bold("Pushing..."));
  await $`wash reg push --insecure ${registry}/${project}:${version} ${destination}`;
}
