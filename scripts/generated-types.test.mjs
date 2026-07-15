import assert from "node:assert/strict";
import test from "node:test";
import { spawnSync } from "node:child_process";
import { readFile, readdir, rename, rm, unlink, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

const generatedDir = resolve("packages/typescript/src/generated");
const rustGenerated = resolve("crates/sol-ledger-schema/src/generated.rs");
const embeddedSchemaDir = resolve("crates/sol-ledger-schema/schemas");

test("published Rust crate embeds the canonical schemas byte for byte", async () => {
  const files = (await readdir(resolve("schemas"))).filter((file) => file.endsWith(".json"));
  assert.ok(files.length > 0, "expected canonical schemas");
  for (const file of files) {
    assert.equal(
      await readFile(resolve(embeddedSchemaDir, file), "utf8"),
      await readFile(resolve("schemas", file), "utf8"),
      `${file} drifted from the canonical schema`,
    );
  }
});

test("generated TypeScript never widens schema objects to explicit any", async () => {
  const files = (await readdir(generatedDir)).filter((file) => file.endsWith(".ts"));
  assert.ok(files.length > 0, "expected checked-in generated TypeScript files");
  for (const file of files) {
    const source = await readFile(resolve(generatedDir, file), "utf8");
    assert.doesNotMatch(source, /\bany\b/, `${file} contains an explicit any type`);
  }
});

test("generated type check rejects stale extra files without deleting them", async () => {
  const staleFile = resolve(generatedDir, "stale-extra.ts");
  await writeFile(staleFile, "// stale generated output\n");
  try {
    const result = spawnSync(process.execPath, ["scripts/generate-types.mjs", "--check"], {
      encoding: "utf8",
    });
    assert.notEqual(result.status, 0, "check unexpectedly accepted a stale generated file");
    assert.match(result.stderr, /unexpected generated types: stale-extra\.ts/);
    assert.equal(await readFile(staleFile, "utf8"), "// stale generated output\n");
  } finally {
    await unlink(staleFile).catch(() => {});
  }
});

test("generation recreates a missing TypeScript output directory", async () => {
  const backup = `${generatedDir}.test-backup`;
  await rename(generatedDir, backup);
  try {
    const result = spawnSync(process.execPath, ["scripts/generate-types.mjs"], { encoding: "utf8" });
    assert.equal(result.status, 0, result.stderr);
    assert.ok((await readdir(generatedDir)).includes("index.ts"));
  } finally {
    await rm(generatedDir, { recursive: true, force: true });
    await rename(backup, generatedDir);
  }
});

test("generation recovers from an uncompilable Rust output", async () => {
  const backup = `${rustGenerated}.test-backup`;
  await rename(rustGenerated, backup);
  await writeFile(rustGenerated, "this is not valid Rust\n");
  try {
    const result = spawnSync(process.execPath, ["scripts/generate-types.mjs"], { encoding: "utf8" });
    assert.equal(result.status, 0, result.stderr);
    assert.match(await readFile(rustGenerated, "utf8"), /pub struct EventEnvelope/u);
  } finally {
    await rm(rustGenerated, { force: true });
    await rename(backup, rustGenerated);
  }
});
