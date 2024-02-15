import { readFileSync, writeFileSync } from "fs";
import { dirname, resolve } from "path";
import { fileURLToPath } from "url";


let ReadPackageVersionCache = null;
export function ReadPackageVersion() {
	ReadPackageVersionCache = ReadPackageVersionCache ?? readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../backend/version"), { encoding: "utf8" });
	return ReadPackageVersionCache;
}

function WriteVersionToPackage(file, version) {
	const pkg = JSON.parse(readFileSync(file));
	pkg.version = version;
	writeFileSync(file, JSON.stringify(pkg, null, "	"));

}

/**
 * Resets all versions back to 0.0.0
 * @param  {...string} packages 
 */
export function ResetVersion(...packages) {
	for (let pkg of packages || ["package.json"]) {
		const packagePath = resolve(pkg);
		WriteVersionToPackage(packagePath, "0.0.0");
	}
}

/**
 * Updates one or more packages to the current version
 * @param  {...string} packages 
 */
export function UpdateVersion(...packages) {
	const version = ReadPackageVersion();

	for (let pkg of packages || ["package.json"]) {
		const packagePath = resolve(pkg);
		WriteVersionToPackage(packagePath, version);
	}
}