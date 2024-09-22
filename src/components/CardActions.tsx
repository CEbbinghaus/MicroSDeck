import { MenuItem, showModal, Menu, ConfirmModal } from "@decky/ui"
import { CardAndGames, MicroSDCard, MicroSDeck } from "../../lib/src"
import { EditCardModal } from "../modals/EditCardModal";
import { UNNAMED_CARD_NAME } from "../const";
import { GamesOnCardModal } from '../modals/GamesOnCardModal';
import { Logger } from '../Logging';

interface CardActionsContextMenuProps {
	microSDeck: MicroSDeck,
	currentCard: MicroSDCard | undefined,
	cardAndGames: CardAndGames
}

/**
 * The context menu for Tab Actions.
 */
export function CardActionsContextMenu({ cardAndGames, currentCard, microSDeck }: CardActionsContextMenuProps) {
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
					onConfirm={async (card: MicroSDCard, nonSteamAdditions: string[], nonSteamDeletions: string[]) => {
						Logger.Debug("Creating Card");
						await microSDeck.updateCard(card);

						await Promise.all(nonSteamAdditions.map(appId => {
							Logger.Debug(`Creating Non-Steam Game ${appId}`);
							
							const appName = collectionStore.deckDesktopApps?.allApps.find(v => v.appid == parseInt(appId))?.display_name ?? "Unknown Game";

							return microSDeck.createGame({ uid: appId, name: appName, is_steam: false, size: 0 })
								.catch(Error => Logger.Error("There was a critical error creating game: \"{Error}\"", { Error }));
						}));
						
						Logger.Debug("Created Non-Steam Games");
						
						Logger.Debug("Linking Many");
						await microSDeck.linkMany(card, nonSteamAdditions);
						
						Logger.Debug("Unlinking Many");
						await microSDeck.unlinkMany(card, nonSteamDeletions);
					}}
					card={{ ...card }}
					games={games}
				/>);
			}}>
				Edit
			</MenuItem>
			<MenuItem onSelected={() => microSDeck.hideCard(card)}>
				Hide
			</MenuItem>
			<MenuItem tone="destructive" disabled={card.uid == currentCard?.uid} onSelected={() => {
				showModal(<ConfirmModal
					bAllowFullSize
					strTitle={`Are you sure you want to delete ${card.name || UNNAMED_CARD_NAME}`}
					onOK={() => microSDeck.deleteCard(card)}
					strOKButtonText="Confirm">
					This cannot be undone. If you insert the card it will be registered again but any changes you have made will be lost.
				</ConfirmModal>);
			}}>
				Delete
			</MenuItem>
		</Menu>
	)
}
