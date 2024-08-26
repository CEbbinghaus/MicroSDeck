import { ReactElement } from "react";
import React from "react";
import { Logger } from "../../src/Logging";

function TagWithCommits({ tag, commits, currentVersion }: { tag: string, commits: Commit[], currentVersion: string }): ReactElement {
	if (commits.length == 0) {
		Logger.Warn("Tag {tag} did not contain any commits. Skipping", { tag: JSON.stringify(tag) });
		return (<></>);
	}

	const latestHash = commits[0].hash.substring(0, 8);

	return (
		<div style={{ margin: "6px auto" }}>
			<h3 style={{ fontWeight: "bolder" }}>{tag || `${currentVersion}@${latestHash} (Pre-Release)`}</h3>
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

export function ChangeLog({ tags, version }: React.PropsWithChildren<{ tags: TaggedCommits[], version: string }>): ReactElement {
	return (
		<>
			{tags.map(({ tag, commits }) => <TagWithCommits tag={tag} commits={commits} currentVersion={version} />)}
		</>
	)
}