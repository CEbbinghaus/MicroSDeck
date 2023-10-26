import { MenuItem, showModal, Menu, ConfirmModal } from "decky-frontend-lib"
import { CardAndGames, MicroSDCard, MicroSDeckManager } from "../../lib/src"
import { EditCardModal } from "../modals/EditCardModal";
import { API_URL, UNAMED_CARD_NAME } from "../const";
import { GamesOnCardModal } from '../modals/GamesOnCardModal';
import { Logger } from '../Logging';

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
				showModal(<GamesOnCardModal
					games={games}
					card={{ ...card }}
				/>);
			}}>
				View Games
			</MenuItem>
			<MenuItem onSelected={() => {
				showModal(<EditCardModal
					onConfirm={(card: MicroSDCard, nonSteamAdditions: string[], nonSteamDeletions: string[]) => {
						microSDeckManager.updateCard(card);

						//* probably want to move this into another method of microSDeckManager or combine it all into updateCard
						nonSteamAdditions.forEach(async appId => {

							//* might wanna tweak this to check responses are good before continuing
							const res1 = await fetch(`${API_URL}/game/${appId}`, {
								method: "POST",
								headers: {
									"Content-Type": "application/json",
								},
								//* i think the collection is only null if user has no shortcuts (non-steam games)
								//* so if we just got the shorcut ids then i think we're good to assert that the collection and the specific game exist here
								body: JSON.stringify({ uid: appId, name: collectionStore.deckDesktopApps!.apps.get(parseInt(appId))!.display_name, is_steam: false, size: 0 }),
								referrerPolicy: "unsafe-url",
							}).catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));

							const res2 = await fetch(`${API_URL}/link`, {
								method: "POST",
								headers: {
									"Content-Type": "application/json",
								},
								body: JSON.stringify({ card_id: card.uid, game_id: appId }),
								referrerPolicy: "unsafe-url",
							}).catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));
						});

						nonSteamDeletions.forEach(async appId => {
							//* api call to remove game from card
						});
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
