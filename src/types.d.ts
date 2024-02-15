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


type DocFile = {path: string, content: string}

declare module "*/docs.codegen" {
	const content: DocFile[];
	export = content;
}
