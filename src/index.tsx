import {
	definePlugin,
	DialogButton,
	Focusable,
	Navigation,
	PanelSection,
	ReorderableEntry,
	ReorderableList,
	ServerAPI,
	showContextMenu,
	staticClasses,
} from "decky-frontend-lib";
import { FaEllipsisH, FaSdCard } from "react-icons/fa";

import PatchAppScreen from "./patch/PatchAppScreen";

import { DOCUMENTATION_PATH, UNAMED_CARD_NAME } from "./const";
import { Logger } from "./Logging";
import React from "react";
import { CardAndGames, MicroSDCard, MicroSDEntryType } from "./lib/Types";
import DocumentationPage from "./pages/Docs";
import { DeckyAPI } from "./lib/DeckyApi";
import { MicroSDeckContextProvider, useMicroSDeckContext } from "./state/MicroSDeckContext";
import { MicroSDeckManager } from "./state/MicoSDeckManager";
import { CardActionsContextMenu } from "./components/CardActions";

declare global {
	let collectionStore: CollectionStore;
}

// function RenderCard({ data }: { data: CardAndGames }) {
// 	Logger.Log("Rendering Card");
// 	const [card, games] = data;

// 	const [name, setName] = useState<string>(card.name);

// 	function onNameChange(e: React.ChangeEvent<HTMLInputElement>) {
// 		setName(e?.target.value);
// 	}

// 	return (
// 		<div>
// 			<TextField tooltip="The name the MicroSD card should be displayed as" value={name} onChange={onNameChange} />
// 			<ScrollPanel>
// 				{games.map(v => (<div>{v.name}</div>))}
// 			</ScrollPanel>
// 		</div>
// 	)
// }
interface EditCardButtonProps {
	microSDeckManager: MicroSDeckManager,
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

function Content(){
	const {currentCardAndGames, cardsAndGames, microSDeckManager} = useMicroSDeckContext();

	const [currentCard] = currentCardAndGames || [undefined];

	const isLoaded = !!cardsAndGames;

	// const [selectedCard, setSelectedCard] = useState<number>(0);

	// Logger.Log("Currently Selected Card: {selectedCard}", { selectedCard });

	// const dropdownOptions = cards?.map(([card], index) => {
	// 	return {
	// 		label: <div>{card.name ?? `Unamed - ${card.uid}`}</div>,
	// 		data: index,
	// 	} as DropdownOption
	// }) ?? [{ label: "Loading...", data: null } as DropdownOption];


	const entries = cardsAndGames?.map(([card]) => {
		return {
			label:
				<div style={{ width: "100%" }} className="tab-label-cont">
					<div style={{ float: "left" }}>
						<FaSdCard size={14} />
					</div>
					<div style={{ marginLeft: "1.2rem", fontSize: 18, fontWeight: "bold" }} className="tab-label">{card.name || UNAMED_CARD_NAME}</div>
					<div style={{ position: "absolute", bottom: 0, left: 0, fontSize: 8, color: "#aaa", whiteSpace: "nowrap" }}>{card.uid}</div>
				</div>
			,
			position: card.position || 0,
			data: { uid: card.uid }
		};
	});

	function CardInteractables({ entry }: {
		entry: ReorderableEntry<MicroSDEntryType>
	}) {
		const cardAndGames = cardsAndGames!.find(([card]) => card.uid == entry.data!.uid)!;
		return (<EditCardButton {...{ cardAndGames, currentCard, microSDeckManager }} />);
	}

	return (
		<>
			<Focusable onMenuActionDescription='Open Docs' onMenuButton={() => { Navigation.CloseSideMenus(); Navigation.Navigate(DOCUMENTATION_PATH); }}>
				<div style={{ margin: "5px", marginTop: "0px" }}>
					Rename, Reorder or Remove MicroSD Cards
				</div>
				{/* <PanelSection>
				<PanelSectionRow>
					<DialogButton
						onClick={() => {
							Router.Navigate(CONFIGURATION_PATH);
							Router.CloseSideMenus();
						}}
					>Open Settings Page</DialogButton>
				</PanelSectionRow>
			</PanelSection> */}
				<PanelSection title="Cards">
					{isLoaded ? (
						<ReorderableList<MicroSDEntryType>
							entries={entries!}
							interactables={CardInteractables}
							onSave={(entries: ReorderableEntry<MicroSDEntryType>[]) => {
								// tabMasterManager.reorderTabs();
								Logger.Log(`Reordered Tabs: [${entries.map(entry => entry.data!.uid).join(", ")}]`)
							}}
						/>
					) : (
						<div style={{ width: "100%", display: "flex", justifyContent: "center", alignItems: "center", padding: "5px" }}>
							Loading...
						</div>
					)}
					{/* 					
					<PanelSectionRow>
						<Dropdown
							focusable={true}
							disabled={!cards || !cards.length}
							rgOptions={dropdownOptions}
							selectedOption={selectedCard}
							onChange={handleDropDownChange}
						/>
					</PanelSectionRow>
					<PanelSectionRow>
						{cards && <RenderCard data={cards[selectedCard]} />}
					</PanelSectionRow> */}
				</PanelSection>
			</Focusable>
		</>
	);
};

export default definePlugin((serverApi: ServerAPI) => {
	serverApi.routerHook.addRoute(DOCUMENTATION_PATH, DocumentationPage, {
		exact: true,
	});

	const microSDeckManager = new MicroSDeckManager();
	//@ts-ignore sssshhhhh
	window.microSDeckManager = microSDeckManager;
	microSDeckManager.init();

	DeckyAPI.SetApi(serverApi);

	const patch = PatchAppScreen(serverApi);

	Logger.Log("Started MicroSDeck");

	return {
		title: <div className={staticClasses.Title}>Example Plugin</div>,
		content:
			<MicroSDeckContextProvider microSDeckManager={microSDeckManager}>
				<Content />
			</MicroSDeckContextProvider>,
		icon: <FaSdCard />,
		onDismount() {
			serverApi.routerHook.removeRoute(DOCUMENTATION_PATH);
			patch && serverApi.routerHook.removePatch('/library/app/:appid', patch);
		},
	};
});
