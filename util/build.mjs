
import { execSync, spawnSync } from 'child_process';
import { join, resolve } from 'path';
import { existsSync, mkdirSync, copyFileSync, statfsSync } from 'fs';
import { Version, UpdateVersion, ResetVersion } from './versioning.mjs';
import { Logger } from './log.mjs';
import { exit } from 'process';

import plugin from "../plugin.json" with { type: "json" };
import { IsCI, SetEnvironment } from './ci.mjs';
const { name: PluginName } = plugin;

if (process.argv.includes('-h') || process.argv.includes('--help')) {
	console.log(
`  __  __ _            ___ ___         _     ___      _ _    _ 
 |  \\/  (_)__ _ _ ___/ __|   \\ ___ __| |__ | _ )_  _(_) |__| |
 | |\\/| | / _| '_/ _ \\__ \\ |) / -_) _| / / | _ \\ || | | / _\` |
 |_|  |_|_\\__|_| \\___/___/___/\\___\\__|_\\_\\ |___/\\_,_|_|_\\__,_|

 by @CEbbinghaus
 `);

	console.log(`
Basic Usage: ./build [flags]

     -h, --help: Prints this help dialogue
    -q, --quiet: Prints only required output
	 -o, --only: Only run the specified part (backend, frontend, collect, copy, upload)
 --skip-backend: Skips building the backend
--skip-frontend: Skips building the frontend
 --skip-collect: Skips copying all the assets into the built output
    --skip-copy: Skips copying the build output to plugin directory (must be run on steamdeck itself)
       --upload: Uploads the build output to the steamdeck. (requires a deploy.json in the root repo directory)
	`)
	process.exit(0);
}

const quiet = process.argv.includes('-q') || process.argv.includes('--quiet');

const tasks = [];
if (process.argv.includes('-o') || process.argv.includes('--only')) {
	let [opt0, opt1] = [process.argv.indexOf('-o'), process.argv.indexOf('--only')];
	
	var index = opt1 > 0 ? opt1 : opt0;
	if (index == process.argv.length - 1) {
		console.error('No argument provided for --only flag');
		process.exit(1);
	}

	for (let i = index + 1; i < process.argv.length; i++) {
		let arg = process.argv[i];
		if(arg.startsWith('-')) break;
		tasks.push(arg);
	}
}

if (tasks.length == 0) {
	tasks.push('backend', 'frontend', 'collect', 'copy');
}

const mapped = {
	"--skip-backend": "backend",
	"--skip-frontend": "frontend",
	"--skip-collect": "collect",
	"--skip-copy": "copy",
}

for(let arg of process.argv) {
	if (mapped[arg]) {
		tasks.splice(tasks.indexOf(mapped[arg]), 1);
	}
}

if (process.argv.includes('--upload') && !tasks.includes('upload')) {
	tasks.push('upload');
}

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
	return (await import(file, { with: { type: "json" } })).default;
}

if (IsCI()) {
	Logger.Log("Running CI related setup");
	SetEnvironment();
}
if (!quiet)
	Logger.Log(`Building plugin ${PluginName}@${Version}`);

if (!existsSync('plugin.json')) {
	console.error('Build script must be run from the root of the repository.');
	process.exit(1);
}

UpdateVersion("package.json", "lib/package.json");

if (tasks.includes('backend')) {
	Logger.Log('Building backend');

	runCommand('cargo build --release', 'backend');
}

if (tasks.includes('frontend')) {
	if (!process.argv.includes('--skip-dependencies')) {
		Logger.Log('Installing dependencies');
		runCommand('pnpm install');
	}

	Logger.Log('Building frontend');
	runCommand('pnpm run bundle');
}

if (tasks.includes('collect')) {
	Logger.Log('Collecting outputs into /build folder');
	mkdirSync('build/dist', { recursive: true });
	mkdirSync('build/bin', { recursive: true });
	copyFileSync('dist/index.js', 'build/dist/index.js');
	copyFileSync('backend/target/x86_64-unknown-linux-musl/release/backend', 'build/bin/backend');
	copyFileSync('main.py', 'build/main.py');
	copyFileSync('plugin.json', 'build/plugin.json');
	copyFileSync('README.md', 'build/README.md');
	copyFileSync('package.json', 'build/package.json');
}

const current_user = execSync("whoami").toString().trim();

const is_local = existsSync(`/home/${current_user}/homebrew`);

if (is_local && tasks.includes('copy')) {
	Logger.Log('Copying build folder to local plugin directory');
	execSync(`sudo rm -rf /home/${current_user}/homebrew/plugins/${PluginName}`);
	execSync(`sudo cp -r build/ /home/${current_user}/homebrew/plugins/${PluginName}`);
	execSync(`sudo chmod 555 /home/${current_user}/homebrew/plugins/${PluginName}`);
} else {
	if (!tasks.includes('copy') && !quiet) {	
		Logger.Log('Skipping copying build folder to local plugin directory');
	} else if (!is_local && !quiet) {
		Logger.Info('Not running on steamdeck');
	}
}

if (tasks.includes('upload')) {
	Logger.Log("Uploading plugin to SteamDeck");

	var deployJsonPath = join(basePath, 'deploy.json');
	try {
		statfsSync(deployJsonPath)
	} catch (e) {
		Logger.Error(`deploy.json not found under \"${basePath}\" Cannot deploy without it`);
		exit(1);
	}

	const { host, user, keyfile } = await importJson(deployJsonPath);

	if (!host || !user) {
		Logger.Error('malformed deploy.json. missing host and user fields');
		exit(1);
	}

	// ping host to make sure its avaliable
	try {
		execSync(`ping -c 1 ${host}`);
	} catch (e) {
		Logger.Error(`Could not connect to ${host}`);
		exit(1);
	}

	const deployPath = `/home/${user}/homebrew/plugins/${PluginName}`;

	const tmpPath = `/tmp/${Date.now()}`

	let keyfileArg = "";

	if(keyfile) {
		keyfileArg = `-i ${keyfile}`;
	}

	execSync(`ssh ${keyfileArg} ${user}@${host} "[ -d ${deployPath} ] && sudo rm -rf ${deployPath} || exit 0"`);
	execSync(`scp ${keyfileArg} -r build/ ${user}@${host}:"${tmpPath}"`);
	execSync(`ssh ${keyfileArg} ${user}@${host} "sudo mv "${tmpPath}" "${deployPath}" && sudo chmod 555 ${deployPath}"`);
}

ResetVersion("package.json", "lib/package.json");
