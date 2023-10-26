import {
	ConfirmModal,
	Field,
	Focusable,
	ScrollPanel,
	TextField,
	ToggleField,
	quickAccessControlsClasses
} from "decky-frontend-lib";
import React from "react";
import { useState, VFC, useEffect, } from "react";
import { DeckyAPI } from "../lib/DeckyApi";
import { Game, MicroSDCard } from "microsdeck";
import { Logger } from "../Logging";
import { UNAMED_CARD_NAME } from "../const";


type EditCardProps = {
	closeModal?: () => void,
	onConfirm: (card: MicroSDCard) => void,
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
		onConfirm({ ...card, name, hidden });
		closeModal!();
	}

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
			<Focusable>
				<ScrollPanel>
					<div style={{ padding: "8px 0px 1px" }} className={quickAccessControlsClasses.PanelSectionTitle}>
						Games
					</div>
					<ul style={{margin: 0, padding: 0}}>
						{
							games.map(v => (<li>{v.name}</li>))
						}
					</ul>
				</ScrollPanel>
			</Focusable>
		</ConfirmModal>
	);
};
