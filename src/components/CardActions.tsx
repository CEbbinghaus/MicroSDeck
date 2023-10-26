import { MenuItem, showModal, Menu, ConfirmModal } from "decky-frontend-lib"
import { CardAndGames, MicroSDCard, MicroSDeckManager } from "../../lib/src"
import { EditCardModal } from "../modals/EditCardModal";
import { UNAMED_CARD_NAME } from "../const";

interface CardActionsContextMenuProps {
	microSDeckManager: MicroSDeckManager,
	currentCard: MicroSDCard | undefined,
	cardAndGames: CardAndGames
}

/**
 * The context menu for Tab Actions.
 */
export function CardActionsContextMenu({ cardAndGames, currentCard, microSDeckManager }: CardActionsContextMenuProps) {
	const [card, games] = cardAndGames;

	return (
		<Menu label="Actions">
			<MenuItem onSelected={() => {
				showModal(<EditCardModal
					onConfirm={(card: MicroSDCard) => {
						microSDeckManager.updateCard(card);
					}}
					card={{ ...card }}
					games={games}
				/>);
			}}>
				Edit
			</MenuItem>
			<MenuItem onSelected={() => microSDeckManager.hideCard(card)}>
				Hide
			</MenuItem>
			<MenuItem tone="destructive" disabled={card.uid == currentCard?.uid} onSelected={() => {
				showModal(<ConfirmModal
					bAllowFullSize
					strTitle={`Are you sure you want to delete ${card.name || UNAMED_CARD_NAME}`}
					onOK={() => microSDeckManager.deleteCard(card)}
					strOKButtonText="Confirm">
					This cannot be undone. If you insert the card it will be registered again but any changes you have made will be lost.
				</ConfirmModal>);
			}}>
				Delete
			</MenuItem>
		</Menu>
	)
}
