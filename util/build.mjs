
import { execSync, spawnSync } from 'child_process';
import { join, resolve } from 'path';
import { existsSync, mkdirSync, copyFileSync, statfsSync } from 'fs';
import { Version, UpdateVersion, ResetVersion } from './versioning.mjs';
import { Logger } from './log.mjs';
import { exit } from 'process';

const basePath = resolve(process.cwd());

/**
 * @param {string} command 
 * @param {string} directory 
 */
function runCommand(command, directory = "") {
	const args = command.split(' ');
	var output = spawnSync(args[0], args.slice(1), { cwd: join(basePath, directory), encoding: 'utf-8' });

	if (output.status != 0) {
		Logger.Error(output.stderr);
		exit(1);
	}

	return output;
}

async function importJson(file) {
	return (await import(file, { assert: { type: "json" } })).default;
}

const { name: PluginName } = await importJson(join(basePath, "plugin.json"));

Logger.Log(`Building plugin ${PluginName}@${Version}`);

if (!existsSync('plugin.json')) {
	console.error('Build script must be run from the root of the repository.');
	process.exit(1);
}

UpdateVersion("package.json", "lib/package.json");

if (!process.argv.includes('--skip-backend')) {
	Logger.Log('Building backend');

	runCommand('cargo build --release', 'backend');
}

if (!process.argv.includes('--skip-frontend')) {
	if (!process.argv.includes('--skip-dependencies')) {
		Logger.Log('Installing dependencies');
		runCommand('pnpm install');
	}

	Logger.Log('Building frontend');
	runCommand('pnpm run build');
}

if (!process.argv.includes('--skip-collect')) {
	Logger.Log('Collecting outputs into /build folder');
	mkdirSync('build/dist', { recursive: true });
	mkdirSync('build/bin', { recursive: true });
	copyFileSync('dist/index.js', 'build/dist/index.js');
	copyFileSync('backend/target/release/backend', 'build/bin/backend');
	copyFileSync('main.py', 'build/main.py');
	copyFileSync('plugin.json', 'build/plugin.json');
	copyFileSync('README.md', 'build/README.md');
	copyFileSync('package.json', 'build/package.json');
}

const is_local = existsSync('/home/deck/homebrew');

if (!is_local) {
	Logger.Info('Not running on steamdeck');
}

if (is_local && !process.argv.includes('--skip-copy')) {
	Logger.Log('Copying build folder to local plugin directory');
	execSync(`sudo rm -rf /home/deck/homebrew/plugins/${PluginName}`);
	execSync(`sudo cp -r build/ /home/deck/homebrew/plugins/${PluginName}`);
	execSync(`sudo chmod 555 /home/deck/homebrew/plugins/${PluginName}`);
} else {
	Logger.Log('Skipping copying build folder to local plugin directory');
}

if (process.argv.includes('--upload')) {
	Logger.Log("Uploading plugin to SteamDeck");

	if (!statfsSync(join(basePath, 'deploy.json'))) {
		Logger.Error("deploy.json not found. Cannot deploy without it");
		exit(1);
	}

	const { host, user, keyfile } = await importJson(join(basePath, "deploy.json"));

	const deployPath = `/home/${user}/homebrew/plugins/${PluginName}`;

	const tmpPath = `/tmp/${Date.now()}`

	execSync(`ssh -i ${keyfile} ${user}@${host} "[ -d ${deployPath} ] && sudo rm -rf ${deployPath} || exit 0"`);
	execSync(`scp -i ${keyfile} -r build/ ${user}@${host}:"${tmpPath}"`);
	execSync(`ssh -i ${keyfile} ${user}@${host} "sudo mv "${tmpPath}" "${deployPath}" && sudo chmod 555 ${deployPath}"`);
}

ResetVersion("package.json", "lib/package.json");
