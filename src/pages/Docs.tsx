import { ReactElement } from "react";
import { FaBook } from "react-icons/fa";
import { SidebarNavigation } from "decky-frontend-lib";
import { DocPage } from "../components/DocPage";
//@ts-ignore typescript being dumb, watch out for a rollup warning complatining about react/jsx-runtime
import Markdown from "react-markdown";
import { DOCUMENTATION_PATH } from "../const";

//@ts-ignore This gets codegenerated at build time 
import docs from './docs.codegen';

// The docs constant as they are baked into the bundle.
// This is more efficient since it precalculates the markdown and doesn't have to do it at every render
const docPages = docs.map(({ path, content }) => {
	return {
		title: path,
		content: <DocPage content={<Markdown>{content}</Markdown>} />,
		route: `${DOCUMENTATION_PATH}/${path.toLowerCase().replace(/ /g, "-")}`,
		icon: <FaBook />,
		hideTitle: true
	};
});

export default function Docs(): ReactElement {
	return (
		<SidebarNavigation
			title="TabMaster Docs"
			showTitle
			pages={docPages}
		/>
	);
};