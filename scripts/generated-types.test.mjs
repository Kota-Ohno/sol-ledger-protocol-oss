import assert from "node:assert/strict";
import test from "node:test";
import { spawnSync } from "node:child_process";
import { readFile, readdir, unlink, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

const generatedDir = resolve("packages/typescript/src/generated");

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
