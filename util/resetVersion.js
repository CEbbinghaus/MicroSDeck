const { WriteVersionToPackage } = require("./common");
const { resolve } = require("path");

const args = process.argv.slice(2);

for(let package of (args || ["package.json"])) {
	const packagePath = resolve(package);
	WriteVersionToPackage(packagePath, "0.0.0");
}
