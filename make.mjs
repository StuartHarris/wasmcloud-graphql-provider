#!/usr/bin/env zx
$.verbose = false;

const schedule = getSchedule(
  parse(
    // don't fail if monobuild is not installed
    (await $`monobuild diff || echo $'interface:\nactor:\nprovider:'`).stdout
  )
);

$.verbose = true;
for (const dep of schedule) {
  cd(dep);
  await $`./make.mjs ${getArgs()}`;
}

function parse(diff) {
  return diff.split("\n").reduce((acc, line) => {
    const [vertex, adjacents] = line.split(":");
    if (vertex) {
      acc[vertex] = adjacents
        .split(",")
        .map((a) => a.trim())
        .filter((a) => a);
    }
    return acc;
  }, {});
}

function getSchedule(adjacencyList) {
  return Object.keys(adjacencyList)
    .map((entryPoint) => {
      const result = [];
      const visited = {};
      (function dfs(vertex) {
        if (!vertex) return null;
        visited[vertex] = true;
        result.push(vertex);
        adjacencyList[vertex].forEach((neighbour) => {
          if (!visited[neighbour]) {
            return dfs(neighbour);
          }
        });
      })(entryPoint);
      return result.reverse();
    }) // depth-first
    .flatMap((list) => list) // flatten
    .filter((vertex, index, self) => self.indexOf(vertex) === index); // unique
}

function getArgs() {
  return Object.keys(argv)
    .filter((k) => k !== "_")
    .flatMap((a) => [`--${a}`, argv[a]]);
}
