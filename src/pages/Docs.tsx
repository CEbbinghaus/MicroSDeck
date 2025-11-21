import { ReactElement } from "react";
import { FaBook } from "react-icons/fa";
import { SidebarNavigation, SidebarNavigationPage } from "@decky/ui";
import { DocPage } from "../components/DocPage";
import { DOCUMENTATION_PATH } from "../const";

//@ts-ignore This gets codegenerated at build time 
import docs from './docs.codegen';

function GetDocHirachyLevel(doc: DocFile): number {
	if (doc.name == "index") return -1;
	return (doc.name.match(/_/g)?.length ?? 0);
}

function GroupDocsByHirachy(docs: DocFile[]): Map<number, DocFile[]> {
	const map = new Map<number, DocFile[]>();
	for (const doc of docs) {
		const level = GetDocHirachyLevel(doc);
		if (!map.has(level)) {
			map.set(level, []);
		}
		map.get(level)!.push(doc);
	}
	return map;
}

function RenderDocsIntoPages() {
	const hirachyMap = GroupDocsByHirachy(docs);
	
	const pages: (SidebarNavigationPage | 'separator')[] = [];

	const levels = Array.from(hirachyMap.keys()).sort((a, b) => a - b);
	
	for (let i = 0; i < levels.length; i++) {
		if (i > 0) {
			pages.push('separator');
		}
		const level = levels[i];
		const docsAtLevel = hirachyMap.get(level)!.sort((a, b) => a.name.localeCompare(b.name));
		for (const doc of docsAtLevel) {
			pages.push({
				title: (doc.name == "index" ? "Home" : doc.name).replace(/_/g, ' ').trim(),
				//@ts-expect-error MDX & React don't play amazingly together... sadly
				content: <DocPage content={window.SP_REACT.createElement(doc.content, null)} />,
				route: `${DOCUMENTATION_PATH}/${doc.name.toLowerCase()}`,
				icon: <FaBook />,
				hideTitle: true
			});
		}
	}
	return pages;
}

// The docs are constant as they are baked into the bundle.
// This is more efficient since it precalculates the pages and doesn't have to do it at every render
const docPages = RenderDocsIntoPages();

export default function Docs(): ReactElement {
	return (
		<SidebarNavigation
			title="MicroSDeck Docs"
			showTitle
			pages={docPages}
		/>
	);
};