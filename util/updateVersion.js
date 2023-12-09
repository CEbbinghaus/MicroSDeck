const { WriteVersionToPackage, ReadPackageVersion } = require("./common");
const { resolve } = require("path");

const args = process.argv.slice(2);

const version = ReadPackageVersion();

console.log(`Updating Version to ${version}`);

for(let package of (args || ["package.json"])) {
	const packagePath = resolve(package);
	WriteVersionToPackage(packagePath, version);
}
