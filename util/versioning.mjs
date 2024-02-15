import { readFileSync, writeFileSync } from "fs";
import { dirname, resolve } from "path";
import { fileURLToPath } from "url";

export const Version = ReadPackageVersion();
function ReadPackageVersion() {
	return readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../backend/version"), { encoding: "utf8" });
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
	for (let pkg of packages || ["package.json"]) {
		const packagePath = resolve(pkg);
		WriteVersionToPackage(packagePath, Version);
	}
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
	// The script was run directly.

	let files = process.argv.slice(3);
	if (files.length == 0) {
		files = ["package.json"];
	}

	switch (process.argv[2]) {
		case "reset":
			console.log(`Resetting ${files.join(", ")}`);
			ResetVersion(...files);
			break;
		case "update":
			console.log(`Updating ${files.join(", ")} to version ${Version}`);
			UpdateVersion(...files);
			break;
		default:
			console.log("Invalid argument provided. Must be one of 'reset' | 'update'");
			process.exit(1);
	}
}