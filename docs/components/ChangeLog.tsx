import { ReactElement } from "react";
import React from "react";

function TagWithCommits({ tag, commits }: { tag: string, commits: Commit[] }): ReactElement {
	return (
		<div style={{margin: "6px auto"}}>
			<h3 style={{fontWeight: "bolder"}}>{tag || "Prerelease"}</h3>
			<ul>
			{
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