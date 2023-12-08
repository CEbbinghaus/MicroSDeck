const { WriteVersionToPackage } = require("./common");
const { readFileSync, writeFileSync } = require("fs");
const { resolve } = require("path");

const args = process.argv.slice(2);

const version = readFileSync(resolve(__dirname, "../version"), {encoding: "utf8"});

console.log(`Updating Version to ${version}`);

for(let package of (args || ["package.json"])) {
	const packagePath = resolve(package);
	WriteVersionToPackage(packagePath, version);
}
