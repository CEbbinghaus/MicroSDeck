import { ReactElement } from "react";
import React from "react";

function TagWithCommits({ tag, commits }: { tag: string, commits: Commit[] }): ReactElement {
	return (
		<>
			{tag || "Prerelease"}
			{
				commits.map(v => (
					<>
						* {v.message}<br />
					</>
				))
			}
		</>
	)
}

export function ChangeLog({ tags }: React.PropsWithChildren<{ tags: TaggedCommits[] }>): ReactElement {
	return (
		<>
			{tags.map(({ tag, commits }) => <TagWithCommits tag={tag} commits={commits} />)}
		</>
	)
}