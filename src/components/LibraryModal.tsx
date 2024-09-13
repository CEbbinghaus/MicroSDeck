import React, { ReactElement, useEffect, useRef, useState } from 'react';
import { FaSdCard } from 'react-icons/fa';
import { Logger } from '../Logging';
import { API_URL, UNNAMED_CARD_NAME } from '../const';
import { useCardsForGame, useMicroSDeckContext } from "../../lib/src"
import { findModule } from "@decky/ui"

const logger = Logger.Child({ module: "patching" });

export default function LibraryModal({ appId: gameId }: { appId: string }): ReactElement {
	const { cards } = useCardsForGame({ url: API_URL, logger: Logger, gameId });
	const { currentCardAndGames } = useMicroSDeckContext();
	const [ currentCard ] = (currentCardAndGames || [undefined]);

	var ref = useRef();

	const bottomMargin = 4;
	const [top, setTop] = useState<number>(210);

	useEffect(() => {
		if(!cards) {
			return;
		}
		
		logger.Debug("Processing Bounds");

		if (!ref || !ref.current) {
			logger.Debug("Couldn't get reference to HTML element");
			return;
		}
		
		const element = (ref.current as unknown as HTMLElement);
		const doc = element.getRootNode() as Document;

		const module = findModule(
			(mod) => typeof mod === 'object' && mod?.Header && mod?.AppDetailsOverviewPanel
		);

		const className = module.Header;
		logger.Debug("Found Header Class under {className}", {className});

		const imageWindow = doc.querySelector(`[class^='${className}']`);

		if (!imageWindow)
		{
			logger.Warn("Unable to retrieve the Header under class \"{className}\"", {className});
			return;
		}

		const imageWindowBounds = imageWindow?.getBoundingClientRect();
		const elementBounds = element.getBoundingClientRect()

		if (!imageWindowBounds || !elementBounds) {
			logger.Debug("Couldn't calculate bounds of image or element\nimage: {imageWindowBounds}\nelement: {elementBounds}", {imageWindowBounds, elementBounds});
			return;
		}


		const topOffset = imageWindowBounds.height - elementBounds.height - bottomMargin;
		setTop(topOffset);
		logger.Debug("Set Top to {topOffset}. Banner Height: {bannerHeight}, Element Height: {elementHeight}, Desired Bottom Margin: {bottomMargin}", {topOffset, bannerHeight: imageWindowBounds.height, elementHeight: elementBounds.height, bottomMargin})
		logger.Log("Set Top For Window bacuse of bounds", { imageWindowBounds });
	}, [cards]);

	if (!cards) {
		logger.Debug("Unable to find Card");
		return (<></>);
	}

	if (!cards.length) {
		logger.Debug("No MicroSD card could be found for {appId}", { appId: gameId });
		return (<></>);
	}


	return (
		<div
			//@ts-ignore
			ref={ref}
			className="microsdeck-app-modal"
			style={{ position: 'absolute', height: 30, top, left: '20px', display: "flex", flexDirection: "row", gap: "8px", flexWrap: "nowrap", justifyContent: "flex-start"}}
		>
			{cards.map(card => (<CardLabel cardName={card.name || UNNAMED_CARD_NAME} isCurrent={card.uid == currentCard?.uid}/>))}
		</div>
	);
}

function CardLabel({ cardName, isCurrent }: { cardName: string, isCurrent: boolean }): ReactElement {
	return (
		<div style={{ padding: "0.4em", borderRadius: "6px", backgroundColor: isCurrent ? "#51bd5c" : "#0c131b"}}>
			<div style={{ float: "left" }}>
				<FaSdCard size={18} />
			</div>
			<div style={{ marginLeft: "1.4rem", lineHeight: "18px", fontSize: 18, fontWeight: "bold" }} className="tab-label">
				{cardName}
			</div>
		</div>
	)
}
