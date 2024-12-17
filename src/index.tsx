import {
	definePlugin,
	DialogButton,
	Focusable,
	Navigation,
	PanelSection,
	ReorderableEntry,
	ReorderableList,
	showContextMenu,
	staticClasses,
} from "@decky/ui";
import { routerHook } from '@decky/api';
import { FaEllipsisH, FaSdCard, FaStar } from "react-icons/fa";
import { GiHamburgerMenu } from "react-icons/gi";
import PatchAppScreen from "./patch/PatchAppScreen";
import { API_URL, DOCUMENTATION_PATH, UNNAMED_CARD_NAME } from "./const";
import { Logger } from "./Logging";
import React from "react";
import Docs from "./pages/Docs";
import { MicroSDeck, MicroSDeckContextProvider, useMicroSDeckContext, CardAndGames, MicroSDCard, IsMatchingSemver } from "../lib/src";
import { CardActionsContextMenu } from "./components/CardActions";
import { backend } from "../lib/src";
import { version as libVersion } from "../lib/src";
import { version } from "../package.json";
import { fetchSetSetting } from "../lib/src/backend";

if (!IsMatchingSemver(libVersion, version)) {
	throw new Error("How the hell did we get here???");
}

declare global {
	let collectionStore: CollectionStore;
}

interface EditCardButtonProps {
	microSDeck: MicroSDeck,
	currentCard: MicroSDCard | undefined,
	cardAndGames: CardAndGames
}
function EditCardButton(props: EditCardButtonProps) {
	const onClick = () => {
		showContextMenu(<CardActionsContextMenu {...props} />);
	}
	return (
		<DialogButton
			style={{ height: "40px", minWidth: "40px", width: "40px", display: "flex", justifyContent: "center", alignItems: "center", padding: "10px", marginRight: "8px" }}
			onClick={onClick}
			onOKButton={onClick}
			onOKActionDescription="Open Card Options"
		>
			<FaEllipsisH />
		</DialogButton>
	)
}

function Content() {
	const { currentCardAndGames, cardsAndGames, microSDeck, frontendSettings } = useMicroSDeckContext();

	const [currentCard] = currentCardAndGames || [undefined];

	const isLoaded = !!cardsAndGames;

	const entries = cardsAndGames?.sort(([a], [b]) => a.position - b.position).map(([card], index) => {
		const currentCardMark = card.uid === currentCard?.uid ? (<small style={{ marginLeft: "0.5em" }}><FaStar size={12} /></small>) : "";

		return {
			label:
				<div style={{ width: "100%" }} className="tab-label-cont">
					<div style={{ float: "left" }}>
						<FaSdCard size={14} />
					</div>
					<div style={{ marginLeft: "1.2rem", fontSize: 18, fontWeight: "bold" }} className="tab-label">{card.name || UNNAMED_CARD_NAME}{currentCardMark}</div>
					<div style={{ position: "absolute", bottom: 0, left: 0, fontSize: 8, color: "#aaa", whiteSpace: "nowrap" }}>{card.uid}</div>
				</div>
			,
			position: index,
			data: card
		};
	});

	function CardInteractables({ entry }: {
		entry: ReorderableEntry<MicroSDCard>
	}) {
		const cardAndGames = cardsAndGames!.find(([card]) => card.uid == entry.data!.uid)!;
		return (<EditCardButton {...{ cardAndGames, currentCard, microSDeck: microSDeck }} />);
	}

	let docs_card = (<></>);

	if (frontendSettings && frontendSettings.dismissed_docs === false) {
		docs_card = (
			<PanelSection title="Docs">
				<div style={{ margin: "5px", marginTop: "0px" }}>
					Open the documentation to learn how to use this plugin, For this use the context button <GiHamburgerMenu />
				</div>
				<DialogButton
					style={{ width: "100%" }}
					onOKButton={() => { fetchSetSetting({ url: API_URL, logger: Logger, setting_name: "frontend:dismissed_docs", value: true }); }}
					onOKActionDescription="Dismiss Docs Reminder">Dismiss</DialogButton>
			</PanelSection>
		);
	}

	return (
		<>
			<Focusable onMenuActionDescription='Open Docs' onMenuButton={() => { Navigation.CloseSideMenus(); Navigation.Navigate(DOCUMENTATION_PATH); }}>
				<div style={{marginTop: "25px"}} ></div>
				{docs_card}
				<div style={{ margin: "5px", marginTop: "0px" }}>
					Edit MicroSD Cards
				</div>
				<PanelSection title="Cards">
					{isLoaded ? (
						<ReorderableList<MicroSDCard>
							entries={entries!}
							interactables={CardInteractables}
							onSave={async (entries: ReorderableEntry<MicroSDCard>[]) => {
								await backend.fetchUpdateCards({
									url: API_URL, logger: Logger, cards: entries.map(v => {
										v.data!.position = v.position;
										return v.data!;
									})
								});

								Logger.Log(`Reordered Tabs`)
							}}
						/>
					) : (
						<div style={{ width: "100%", display: "flex", justifyContent: "center", alignItems: "center", padding: "5px" }}>
							Loading...
						</div>
					)}
				</PanelSection>
			</Focusable>
		</>
	);
};

export default definePlugin(() => {

	if (window.MicroSDeck) {
		window.MicroSDeck.destruct();
	}
	window.MicroSDeck = new MicroSDeck({ url: API_URL, logger: Logger });

	Logger.Log("Started MicroSDeck");

	const patch = PatchAppScreen(window.MicroSDeck);

	routerHook.addRoute(DOCUMENTATION_PATH, () => (
		<MicroSDeckContextProvider microSDeck={window.MicroSDeck || (() => {throw "MicroSDeck not initialized";})()}>
			<Docs />
		</MicroSDeckContextProvider>));

	return {
		title: <div className={staticClasses.Title}>MicroSDeck</div>,
		content:
			<MicroSDeckContextProvider microSDeck={window.MicroSDeck}>
				<Content />
			</MicroSDeckContextProvider>,
		icon: <FaSdCard />,
		onDismount() {
			window.MicroSDeck?.destruct();
			window.MicroSDeck = undefined;

			routerHook.removeRoute(DOCUMENTATION_PATH);
			patch && routerHook.removePatch('/library/app/:appid', patch);
		},
	};
});
