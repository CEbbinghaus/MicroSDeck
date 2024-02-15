import { ReactElement } from "react";
import { FaBook } from "react-icons/fa";
import { SidebarNavigation } from "decky-frontend-lib";
import { DocPage } from "../components/DocPage";
//@ts-ignore typescript being dumb, watch out for a rollup warning complatining about react/jsx-runtime
import Markdown from "react-markdown";
import { DOCUMENTATION_PATH } from "../const";

//@ts-ignore This gets codegenerated at build time 
import docs from './docs.codegen';

const renderedDocs = docs.map(v => { return { ...v, content: (<Markdown>{v.content}</Markdown>) } });


export default function DocumentationPage(): ReactElement {
	const docPages = renderedDocs.map(({ path, content }) => {
		return {
			title: path,
			content: <DocPage content={content} />,
			route: `${DOCUMENTATION_PATH}/${path.toLowerCase().replace(/ /g, "-")}`,
			icon: <FaBook />,
			hideTitle: true
		};
	});

	return (
		<SidebarNavigation
			title="TabMaster Docs"
			showTitle
			pages={docPages}
		/>
	);
};