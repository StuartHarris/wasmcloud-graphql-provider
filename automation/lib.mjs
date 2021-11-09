#!/usr/bin/env zx

export async function retry(
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

export async function ifChanged(inputDir, outputDir, fn) {
  const shaFile = `${outputDir}/.sha`;
  let previous = "";
  $.verbose = false;
  try {
    previous = (await fs.readFile(shaFile)).toString().trimEnd("\n");
  } catch {}
  let current = (await $`dirsh ${inputDir}`).stdout.trimEnd("\n");
  $.verbose = true;
  console.log({ previous, current });
  if (previous !== current) {
    await fn();
    await fs.writeFile(shaFile, current);
  }
}

export function getArch() {
  const operating_systems = {
    darwin: "macos",
    linux: "linux",
  };
  const architectures = {
    x64: "x86_64", // todo add ARM
  };
  return `${architectures[os.arch()]}-${operating_systems[os.platform()]}`;
}

export async function getProject() {
  $.verbose = false;
  const meta = JSON.parse(await $`cargo metadata --no-deps --format-version 1`);
  $.verbose = true;
  return meta.packages[0];
}

export function step(msg) {
  console.log(chalk.blue.bold(`----\n${msg}`));
}

export function setColors() {
  process.env.CARGO_TERM_COLOR = "always";
  process.env.FORCE_COLOR = "3";
}
