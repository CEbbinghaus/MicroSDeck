const { readFileSync, writeFileSync } = require("fs");
const { join } = require("path");

const packagePath = join(process.cwd(), "package.json");

const package = JSON.parse(readFileSync(packagePath));
package.version = "0.0.0";
writeFileSync(packagePath, JSON.stringify(package, null, "	"));
