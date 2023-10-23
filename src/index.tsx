import {
	definePlugin,
	DialogButton,
	Focusable,
	Navigation,
	PanelSection,
	ReorderableEntry,
	ReorderableList,
	ScrollPanel,
	ServerAPI,
	showModal,
	staticClasses,
	TextField,
} from "decky-frontend-lib";
import { useState, VFC } from "react";
import { FaPen, FaSdCard } from "react-icons/fa";

import PatchAppScreen from "./patch/PatchAppScreen";

import { DOCUMENTATION_PATH } from "./const";
import { Logger } from "./Logging";
import { GetCardsAndGames, SetNameForMicroSDCard } from "./hooks/backend";
import React from "react";
import { CardAndGames, MicroSDEntryType } from "./lib/Types";
import { EditCardModal } from "./modals/EditCardModal";
import DocumentationPage from "./pages/Docs";
import { DeckyAPI } from "./lib/DeckyApi";

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

function EditCardButton({ card }: { card: CardAndGames }) {


	const onClick = () => {
		showModal(<EditCardModal
			onConfirm={(cardId: string, name: string) => {
				Logger.Log("Changed Card {cardId} to name \"{name}\"", { cardId, name });
				SetNameForMicroSDCard(cardId, name);
			}}
			cardId={card[0].uid}
			cardName={card[0].name}
		/>);
	}
	return (
		<DialogButton
			style={{ height: "40px", minWidth: "40px", width: "40px", display: "flex", justifyContent: "center", alignItems: "center", padding: "10px", marginRight: "8px" }}
			onClick={onClick}
			onOKButton={onClick}
			onOKActionDescription="Edit Card"
		>
			<FaPen />
		</DialogButton>
	)
}

const Content: VFC<{ serverAPI: ServerAPI }> = ({ }) => {
	const { cards, refresh } = GetCardsAndGames();

	const isLoaded = !!cards;

	// const [selectedCard, setSelectedCard] = useState<number>(0);

	// Logger.Log("Currently Selected Card: {selectedCard}", { selectedCard });

	// const dropdownOptions = cards?.map(([card], index) => {
	// 	return {
	// 		label: <div>{card.name ?? `Unamed - ${card.uid}`}</div>,
	// 		data: index,
	// 	} as DropdownOption
	// }) ?? [{ label: "Loading...", data: null } as DropdownOption];


	const entries = cards?.map(([card]) => {
		return {
			label:

				<div className="tab-label-cont">
					<div style={{ height: "80%", float: "left" }}>
						<FaSdCard />
					</div>
					<div style={{ marginLeft: "1.2em", fontSize: 18, fontWeight: "bold" }} className="tab-label">{card.name}</div>
					<div style={{ position: "relative", bottom: 0, left: 0, fontSize: 8, color: "#aaa", whiteSpace: "nowrap" }}>{card.uid}</div>
				</div>
			,
			position: card.position || 0,
			data: { uid: card.uid }
		};
	});

	function CardInteractables({ entry }: {
		entry: ReorderableEntry<MicroSDEntryType>
	}) {
		const card = cards!.find(([card]) => card.uid == entry.data!.uid)!;
		return (<EditCardButton {...{ card }} />);
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

	DeckyAPI.SetApi(serverApi);

	const patch = PatchAppScreen(serverApi);

	Logger.Log("Started MicroSDeck");

	return {
		title: <div className={staticClasses.Title}>Example Plugin</div>,
		content: <Content serverAPI={serverApi} />,
		icon: <FaSdCard />,
		onDismount() {
			serverApi.routerHook.removeRoute(DOCUMENTATION_PATH);
			patch && serverApi.routerHook.removePatch('/library/app/:appid', patch);
		},
	};
});
