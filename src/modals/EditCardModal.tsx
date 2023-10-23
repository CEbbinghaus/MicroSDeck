import {
	ConfirmModal,
	Field,
	TextField,
	quickAccessControlsClasses
} from "decky-frontend-lib";
import React from "react";
import { useState, VFC, useEffect, } from "react";
import { DeckyAPI } from "../lib/DeckyApi";


type EditCardProps = {
	closeModal?: () => void,
	onConfirm: (cardId: string, name: string) => void,
	cardName: string | undefined,
	cardId: string
};

/**
 * The modal for editing and creating custom tabs.
 */
export const EditCardModal: VFC<EditCardProps> = ({ cardId, cardName, onConfirm, closeModal }) => {
	const [name, setName] = useState<string>(cardName ?? '');
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
		onConfirm(cardId, name);
		closeModal!();
	}

	return (
		<>
			<div>
				<ConfirmModal
					bAllowFullSize
					onCancel={closeModal}
					onEscKeypress={closeModal}
					strTitle={"Editing" + (name ?? "Unamed")}
					onOK={onSave}
					strOKButtonText="Save"
				>
					<div style={{ padding: "4px 16px 1px" }} className="name-field">
						<Field description={
							<>
								<div style={{ paddingBottom: "6px" }} className={quickAccessControlsClasses.PanelSectionTitle}>
									Name
								</div>
								{nameInputElt}
							</>
						} />
					</div>
				</ConfirmModal>
			</div>
		</>
	);
};