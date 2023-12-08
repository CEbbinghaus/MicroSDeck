const { readFileSync, writeFileSync } = require("fs");
const { join } = require("path");

const packagePath = join(process.cwd(), "package.json");
const version = readFileSync(join(__dirname, "../version"), {encoding: "utf8"});

console.log(`Updating Version to ${version}`);

const package = JSON.parse(readFileSync(packagePath));
package.version = version;
writeFileSync(packagePath, JSON.stringify(package, null, "	"));
