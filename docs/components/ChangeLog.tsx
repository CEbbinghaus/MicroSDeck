import { ReactElement } from "react";
import React from "react";
import { Logger } from "../../src/Logging";

function TagWithCommits({ tag, commits }: { tag: string, commits: Commit[] }): ReactElement {
	if (commits.length == 0) {
		Logger.Warn("Tag {tag} did not contain any commits. Skipping", { tag: JSON.stringify(tag) });
		return (<></>);
	}

	if (!tag) {
		Logger.Warn("Tag is not defined. Skipping", { tag: JSON.stringify(tag) });
		return (<></>);
	}

	return (
		<div style={{ margin: "6px auto" }}>
			<h3 style={{ fontWeight: "bolder" }}>{tag}</h3>
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