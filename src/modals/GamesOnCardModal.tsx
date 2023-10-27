import { Focusable, ModalPosition, ScrollPanelGroup, SimpleModal, gamepadDialogClasses, quickAccessControlsClasses } from "decky-frontend-lib";
import { Fragment, VFC } from "react";
import { Game, MicroSDCard } from '../../lib/src';

interface GamesOnCardModalProps {
	card: MicroSDCard;
	games: Game[];
	additions?: { [appId: string]: boolean };
	deletions?: { [appId: string]: boolean };
	closeModal?: () => void;
}

export const GamesOnCardModal: VFC<GamesOnCardModalProps> = ({ card, games, additions, deletions, closeModal }) => {
	const additionIds = additions ?? {};
	const deletionIds = deletions ?? {};

	const padding = '0 8px'
	const steamGames = games.flatMap(game => game.is_steam ? <li style={{ padding }}>{game.name}</li> : []);
	const nonSteamGames = games.flatMap(game => !game.is_steam ? <li style={Object.assign({ padding }, deletionIds[game.uid] ? { background: '#9e1b3452' } : {})}>{game.name}</li> : []);
	const additionElements = Object.keys(additionIds).map(appId => {
		let app;
		return <li style={{ background: '#0c791647', padding }}>
			{!collectionStore.deckDesktopApps ? appId : (app = collectionStore.deckDesktopApps.apps.get(parseInt(appId))) ? app.display_name : appId + ': DELETED FROM STEAM'}
		</li>;
	});

	return <Fragment>
		<style>{`
        .microsdeck-gamesoncardmodal .${gamepadDialogClasses.ModalPosition} {
          padding: 0;
          margin: 0 150px;
        }
      `}</style>
		<SimpleModal active={true}>
			<div className="microsdeck-gamesoncardmodal">
				<ModalPosition>
					<div>
						<h2 style={{ margin: '25px 0 5px' }}>{'Games On Card: ' + card.name || card.uid}</h2>
					</div>
					<Focusable style={{ display: "flex", flexDirection: "column", minHeight: 0, flex: 1, WebkitMaskImage: 'linear-gradient(to bottom, transparent, black 5%)' }}>
						<ScrollPanelGroup
							//@ts-ignore
							focusable={false}
							style={{ flex: 1, minHeight: 0, padding: "12px 0" }}
						>
							<Focusable noFocusRing={true} onActivate={closeModal} onCancel={closeModal}>
								{steamGames.length > 0 && (
									<Fragment>
										<div>
											<div className={quickAccessControlsClasses.PanelSectionTitle}>Steam</div>
											<ul style={{ margin: 0, padding: 0 }}>
												{steamGames}
											</ul>
										</div>
										<br />
										<br />
									</Fragment>
								)}
								{(nonSteamGames.length > 0 || additionElements.length > 0) && (
									<div>
										<div className={quickAccessControlsClasses.PanelSectionTitle}>Non Steam</div>
										<ul style={{ margin: 0, padding: 0 }}>
											{nonSteamGames}
											{additionElements}
										</ul>
									</div>
								)}
							</Focusable>
						</ScrollPanelGroup>
					</Focusable>
				</ModalPosition>
			</div >
		</SimpleModal>
	</Fragment>;
};
