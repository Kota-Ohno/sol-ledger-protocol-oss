import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import addFormats from "ajv-formats";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const ajv = new Ajv2020({ allErrors: true });
addFormats(ajv);
for (const name of ["security-policy", "event-envelope", "artifact-ref", "provenance-edge"]) {
  ajv.addSchema(JSON.parse(fs.readFileSync(path.join(root, "schemas", `${name}.schema.json`), "utf8")));
}

export function validateFixtures() {
  const failures = [];
  for (const expectation of ["valid", "invalid"]) {
    for (const file of fs.readdirSync(path.join(root, "fixtures", expectation)).filter((entry) => entry.endsWith(".json"))) {
      const name = file.replace(/\.json$/, "");
      const validate = ajv.getSchema(`https://sol-ledger.dev/schema/${name}/0.1.0`);
      const actual = validate(JSON.parse(fs.readFileSync(path.join(root, "fixtures", expectation, file), "utf8")));
      if (actual !== (expectation === "valid")) failures.push(`${expectation}/${file}: ${JSON.stringify(validate.errors)}`);
    }
  }
  return failures;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const failures = validateFixtures();
  if (failures.length) {
    console.error(failures.join("\n"));
    process.exitCode = 1;
  } else {
    console.log("All fixtures matched their expected validity.");
  }
}
