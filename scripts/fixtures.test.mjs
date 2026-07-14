import test from "node:test";
import assert from "node:assert/strict";
import { validateFixtures } from "./validate-fixtures.mjs";

test("valid and invalid fixtures are classified correctly", () => {
  assert.deepEqual(validateFixtures(), []);
});
