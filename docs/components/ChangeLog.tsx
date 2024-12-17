import { ReactElement } from "react";
import React from "react";

function TagWithCommits({ tag, commits }: { tag: string, commits: Commit[] }): ReactElement {
	let colorStyle = {};
	if (!tag.match(/\d+\.\d+\.\d+/)) {
		colorStyle = { color: "#b0b0b0" };
	}

	return (
		<div style={{margin: "6px auto"}}>
			<h3 style={{ fontWeight: "bolder", ...colorStyle }}>{tag || "Prerelease"}</h3>
			<ul>
			{
				//TODO: categorize them based on semantic commit
				commits.map(v => (
					<li>
						{v.message}
					</li>
				))
			}
			</ul>
		</div>
	)
}

export function ChangeLog({ tags }: React.PropsWithChildren<{ tags: TaggedCommits[] }>): ReactElement {
	return (
		<>
			{tags.map(({ tag, commits }) => <TagWithCommits tag={tag} commits={commits} />)}
		</>
	)
}