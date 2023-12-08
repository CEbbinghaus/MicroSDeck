
const { readFileSync, writeFileSync } = require("fs");


function WriteVersionToPackage(file, version) {
	const package = JSON.parse(readFileSync(file));
	package.version = version;
	writeFileSync(file, JSON.stringify(package, null, "	"));

}

module.exports = {
	WriteVersionToPackage
};