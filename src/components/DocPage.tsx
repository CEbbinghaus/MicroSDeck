import { Focusable, ModalPosition, ScrollPanelGroup, gamepadDialogClasses } from "decky-frontend-lib";
import React, { ReactElement } from "react";

export function DocPage({ content }: { content: JSX.Element }): ReactElement {
	return (
		<>
			<style>{`
			.microsd-docs table {
				border: 1px solid;
				border-collapse: collapse;
			}

			.microsd-docs th {
				padding: 0 7px;
				border: 1px solid;
			}

			.microsd-docs td {
				padding: 0 7px;
				border: 1px solid;
			}

			.microsd-docs tr:nth-child(odd) {
				background-color: #1B2838;
			}

			.microsd-docs .${gamepadDialogClasses.ModalPosition} {
				padding: 0;
			}

			.microsd-docs > .Panel.Focusable.gpfocuswithin {
				background-color: #868da117;
			}

			.microsd-docs img {
				max-width: 588px;
			}

			.microsd-docs code {
				color: #f1ac4f;
				padding: 2px 4px;
				border-radius: 4px;
			}
      `}</style>
			<div className="microsd-docs">
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