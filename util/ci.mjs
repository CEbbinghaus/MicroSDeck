import { execSync } from "child_process";
import { fileURLToPath } from "url";
import { Logger } from "./log.mjs";
import { env } from "process";

console.log(JSON.stringify(process.env))
console.log(JSON.stringify(env))

export function IsCI() {
	return !!process.env.CI;
}

export function SetEnvironment() {
	Logger.Info("Setting git config")
	execSync("git config --global --add safe.directory /plugin");
}

// If this file is being run rather than imported as a module
if (process.argv[1] === fileURLToPath(import.meta.url)) {
	if (IsCI()) {
		SetEnvironment();
	}
}