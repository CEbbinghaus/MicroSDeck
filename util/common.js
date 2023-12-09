
const { readFileSync, writeFileSync } = require("fs");
const { resolve } = require("path");

function ReadPackageVersion() {
	return readFileSync(resolve(__dirname, "../backend/version"), {encoding: "utf8"});
}

function WriteVersionToPackage(file, version) {
	const package = JSON.parse(readFileSync(file));
	package.version = version;
	writeFileSync(file, JSON.stringify(package, null, "	"));

}

module.exports = {
	WriteVersionToPackage,
	ReadPackageVersion
};