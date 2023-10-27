import {
	ConfirmModal,
	DialogButton,
	DialogCheckbox,
	Field,
	Focusable,
	TextField,
	ToggleField,
	quickAccessControlsClasses,
	showModal
} from "decky-frontend-lib";
import React, { CSSProperties, Fragment, useMemo } from "react";
import { useState, VFC, useEffect, } from "react";
import { BiSolidDownArrow } from "react-icons/bi";
import { DeckyAPI } from "../lib/DeckyApi";
import { Game, MicroSDCard } from "../../lib/src";
import { Logger } from "../Logging";
import { UNAMED_CARD_NAME } from "../const";
import { GamesOnCardModal } from './GamesOnCardModal';
import { GamepadUIAudio } from '../lib/GamepadUIAudio';


type EditCardProps = {
	closeModal?: () => void,
	onConfirm: (card: MicroSDCard, nonSteamAdditions: string[], nonSteamDeletions: string[]) => void,
	card: MicroSDCard,
	games: Game[]
};

/**
 * The modal for editing and creating custom tabs.
 */
export const EditCardModal: VFC<EditCardProps> = ({ card, games, onConfirm, closeModal }) => {

	const [name, setName] = useState<string>(card.name ?? '');
	const [hidden, setHidden] = useState<boolean>(card.hidden);
	const [canSave, setCanSave] = useState<boolean>(false);
	const [additions, setAdditions] = useState<{ [appId: string]: boolean }>({});
	const [deletions, setDeletions] = useState<{ [appId: string]: boolean }>({});
	const nonSteamIdsOnCard = useMemo(() => games.reduce<{ [appId: string]: boolean }>((output, game) => !game.is_steam ? Object.assign(output, { [game.uid]: true }) : output, {}), []);
	const [checkedIds, setCheckedIds] = useState(nonSteamIdsOnCard);

	const nameInputElt = <TextField value={name} onChange={onNameChange} />;

	useEffect(() => {
		setCanSave(!!name);
	}, [name]);

	function onNameChange(e: React.ChangeEvent<HTMLInputElement>) {
		setName(e?.target.value);
	}

	function onSave() {
		if (!canSave) {
			DeckyAPI.Toast("Cannot Save MicroSD Card", "You must provide a valid name");
			return;
		}
		onConfirm({ ...card, name, hidden }, Object.keys(additions), Object.keys(deletions));
		closeModal!();
	}

	function onNonSteamChange(appId: number, checked: boolean) {
		setCheckedIds(currentChecked => ({ ...currentChecked, [appId]: checked }));
		if (checked) {
			if (!nonSteamIdsOnCard[appId]) setAdditions(current => ({ ...current, [appId]: true }));
			if (deletions[appId]) setDeletions(current => (({ [appId]: _, ...rest }) => rest)(current));
		} else {
			if (nonSteamIdsOnCard[appId]) setDeletions(current => ({ ...current, [appId]: true }));
			if (additions[appId]) setAdditions(current => (({ [appId]: _, ...rest }) => rest)(current));
		}
	};
	
	return (
		<ConfirmModal
			bAllowFullSize
			onCancel={closeModal}
			onEscKeypress={closeModal}
			strTitle={`Editing Card: "${(card.name ?? UNAMED_CARD_NAME)}"`}
			onOK={onSave}
			strOKButtonText="Save">
			<Field description={
				<>
					<div style={{ paddingBottom: "6px" }} className={quickAccessControlsClasses.PanelSectionTitle}>
						Name
					</div>
					{nameInputElt}
				</>
			} />
			<ToggleField label="Hidden" checked={hidden} onChange={(checked) => {
				Logger.Log("changed value to: {checked}", { checked });
				setHidden(checked);
			}} />
			<NonSteamPanel
				checkedIds={checkedIds}
				onChange={onNonSteamChange}
				idsOnCard={nonSteamIdsOnCard}
				numAdditions={Object.keys(additions).length}
				numDeletions={Object.keys(deletions).length}
			style={{ marginBottom: "24px" }}
			/>
			<DialogButton onClick={() => {
				showModal(<GamesOnCardModal
					games={games}
					card={{ ...card }}
					additions={additions}
					deletions={deletions}
				/>);
			}}>
				View Games
			</DialogButton>
		</ConfirmModal>
	);
};

interface NonSteamPanelProps {
	checkedIds: { [appId: string]: boolean };
	idsOnCard: { [appId: string]: boolean };
	numAdditions: number;
	numDeletions: number;
	style?: CSSProperties;
	onChange: (appId: number, checked: boolean) => void;
}

//* need to add a way to check if a shorcut no longer exists in steam but still exists on the card
//* probably would be best to just watch for deleteions of shorcuts and automatically handle it in the background and not in the modal at all
/**
 * Section for managing non steam games associated with a card
 */
const NonSteamPanel: VFC<NonSteamPanelProps> = ({ checkedIds, idsOnCard, numAdditions, numDeletions, style, onChange }) => {
	const [isOpen, setIsOpen] = useState(false);
	const [query, setQuery] = useState("");
	const idNamePairs: [number, string][] = useMemo(() => collectionStore.deckDesktopApps ? collectionStore.deckDesktopApps.allApps.map(appOverview => [appOverview.appid, appOverview.display_name]) : [], [collectionStore.deckDesktopApps?.allApps.length]);
	const [filteredPairs, setFilteredPairs] = useState(idNamePairs);

	useEffect(() => {
		setFilteredPairs(idNamePairs.filter(([_, name]) => name.toLowerCase().includes(query.toLowerCase())));
	}, [query]);

	return (
		<>
			<style>{
				`.microsdeck-nonsteampanel .start-focused {
					background-color: #3d4450 !important;
				}`
			}</style>
			<div className="microsdeck-nonsteampanel" style={style}>
				<Focusable
					style={{ margin: "0 calc(-12px - 1.4vw)" }}
					onActivate={() => {
						GamepadUIAudio.AudioPlaybackManager.PlayAudioURL('/sounds/deck_ui_misc_01.wav');
						setIsOpen(isOpen => !isOpen);
					}}
					noFocusRing={true}
					focusClassName="start-focused"
				>
					<div style={{ margin: "0 calc(12px + 1.4vw)" }}>
						<div style={{ display: "flex", alignItems: "center" }}>
							<div style={{ padding: "12px 0", float: "left" }} className={quickAccessControlsClasses.PanelSectionTitle}>
								Manage Non Steam Games
							</div>
							<div style={{ flex: "1", textAlign: 'center' }}>
								<span style={{ fontSize: "12px", lineHeight: "12px", color: "#8b929a" }}>
									{[numAdditions ? `${numAdditions} Addition${numAdditions > 1 ? 's' : ''}` : '', numDeletions ? `${numDeletions} Deletion${numDeletions > 1 ? 's' : ''}` : ''].filter(string => string).join(', ')}
								</span>
							</div>
							<div style={{ paddingRight: "10px", display: "flex", alignItems: "center" }}>
								<BiSolidDownArrow
									style={{
										transform: !isOpen ? "rotate(90deg)" : "",
										transition: "transform 0.2s ease-in-out",
									}}
								/>
							</div>
						</div>
					</div>
				</Focusable>
				{isOpen && (
					<Fragment>
						<div style={{ padding: "10px 18px" }}>
							<div style={{ width: "100%", marginBottom: '10px' }}>
								<TextField
									placeholder='Filter'
									value={query}
									onChange={(e) => { setQuery(e.target.value); }}
									style={{ height: "100%" }}
								/>
							</div>
							{filteredPairs.map(([appId, displayName]) => {
								const onChanged = (checked: boolean) => {
									GamepadUIAudio.AudioPlaybackManager.PlayAudioURL(checked ? '/sounds/deck_ui_switch_toggle_on.wav' : '/sounds/deck_ui_switch_toggle_off.wav');
									onChange(appId, checked);
								};
								return (
									<DialogCheckbox
										checked={checkedIds[appId]}
										onChange={onChanged}
										label={<Fragment>
											<div style={{ flex: 1 }}>{displayName}</div>
											{idsOnCard[appId] &&
												<div style={{
													width: 'auto',
													marginLeft: '25px',
													marginRight: '10px',
													fontSize: "12px",
													color: "#8b929a"
												}}>
													On Card
												</div>}
										</Fragment>}
									/>
								);
							})}
						</div>
					</Fragment>
				)}
				<div style={{
					left: "calc(16px - 1.8vw)",
					right: "calc(16px - 1.8vw)",
					height: "1px",
					background: "#23262e"
				}}
				/>
			</div>
		</>
	);
};