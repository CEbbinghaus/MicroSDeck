declare module "*.svg" {
	const content: string;
	export default content;
}

declare module "*.png" {
	const content: string;
	export default content;
}

declare module "*.jpg" {
	const content: string;
	export default content;
}


type DocFile = { name: string, content: JSX.Element }

declare module "*/docs.codegen" {
	const content: DocFile[];
	export = content;
}

type Commit = {
	hash: string,
	tag: string,
	author: string,
	email: string,
	message: string,
}

type TaggedCommits = {
	tag: string,
	commits: Commit[]
}

declare module "*/changelog.codegen" {
	const content: TaggedCommits[];
	export = content;
}
