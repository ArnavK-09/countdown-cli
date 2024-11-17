#!/usr/bin/env node

const { spawnSync } = require("child_process");

function getExecutablePath() {
  const arch = process.arch;
  let os = process.platform;
  let extension = "";

  if (["win32", "cygwin"].includes(process.platform)) {
    os = "windows";
    extension = ".exe";
  }

  try {
    return require.resolve(
      `rust_cli_for_npx-${os}-${arch}/bin/rust_cli_for_npx${extension}`,
    );
  } catch {
    throw new Error(
      `Couldn't find application binary inside node_modules for ${os}-${arch}`,
    );
  }
}

function main() {
  const args = process.argv.slice(2);
  const processResult = spawnSync(getExecutablePath(), args, {
    stdio: "inherit",
  });
  process.exit(processResult.status ?? 0);
}

main();
