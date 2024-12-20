import { readFileSync, writeFileSync, utimesSync, statSync } from "fs";
import { dirname, resolve } from "path";
import { fileURLToPath } from "url";
import { Logger } from "./log.mjs";

export const Version = ReadPackageVersion();
function ReadPackageVersion() {
	return readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../backend/version"), { encoding: "utf8" }).trim();
}

function WriteVersionToPackage(file, version) {
	// Get last modified time of file
	const { atime, mtime } = statSync(file);

	var value = readFileSync(file, { encoding: "utf-8" });
	const pkg = {...JSON.parse(value), version };
	writeFileSync(file, JSON.stringify(pkg, null, "	"));

	// update file so it doesn't get marked as changed
	utimesSync(file, atime, mtime);
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

// If this file is being run rather than imported as a module
if (process.argv[1] === fileURLToPath(import.meta.url)) {
	// The script was run directly.

	let files = process.argv.slice(3);
	if (files.length == 0) {
		files = ["package.json"];
	}

	switch (process.argv[2]) {
		case "reset":
			Logger.Info(`Resetting ${files.join(", ")}`);
			ResetVersion(...files);
			break;
		case "update":
			Logger.Info(`Updating ${files.join(", ")} to version ${Version}`);
			UpdateVersion(...files);
			break;
		default:
			Logger.Info("Invalid argument provided. Must be one of 'reset' | 'update'");
			process.exit(1);
	}
}