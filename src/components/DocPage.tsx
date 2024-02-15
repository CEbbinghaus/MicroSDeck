import { VFC } from "react";
import { Focusable, ModalPosition, ScrollPanelGroup } from "decky-frontend-lib";
import React from "react";

export const DocPage: VFC<{ content: JSX.Element }> = ({ content }) => {
	return (
		<>
			<div className="tab-master-docs">
				<ModalPosition >
					<Focusable style={{ display: "flex", flexDirection: "column", minHeight: 0 }}>
						<ScrollPanelGroup
							//@ts-ignore
							focusable={false}
							style={{ flex: 1, minHeight: 0, padding: "12px" }}
							scrollPaddingTop={32}
						>
							<Focusable onActivate={() => { }} noFocusRing={true} >
								{content}
							</Focusable>
						</ScrollPanelGroup>
					</Focusable>
				</ModalPosition>
			</div>
		</>
	);
};