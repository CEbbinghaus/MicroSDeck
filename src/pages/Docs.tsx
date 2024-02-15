import { ReactElement } from "react";
import { FaBook } from "react-icons/fa";
import { SidebarNavigation } from "decky-frontend-lib";
import { DocPage } from "../components/DocPage";
import { DOCUMENTATION_PATH } from "../const";

//@ts-ignore This gets codegenerated at build time 
import docs from './docs.codegen';

// The docs are constant as they are baked into the bundle.
// This is more efficient since it precalculates the markdown and doesn't have to do it at every render
const docPages = docs.map(({ path, content }) => {
	return {
		title: (path == "index.md" ? "Main" : path.replace(".mdx", "").replace(".md", "")).trim(),
		//@ts-expect-error MDX & React don't play amazingly together... sadly
		content: <DocPage content={window.SP_REACT.createElement(content, null)} />,
		route: `${DOCUMENTATION_PATH}/${path.toLowerCase().replace(/ /g, "-")}`,
		icon: <FaBook />,
		hideTitle: true
	};
});

export default function Docs(): ReactElement {
	return (
		<SidebarNavigation
			title="MicroSDeck Docs"
			showTitle
			pages={docPages}
		/>
	);
};