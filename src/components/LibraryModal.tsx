import React, { ReactElement, useEffect, useRef, useState} from 'react';
import { FaSdCard } from 'react-icons/fa';
import { Logger } from '../Logging';
import { API_URL, UNAMED_CARD_NAME } from '../const';
import { useCardsForGame } from "../../lib/src"

export default function LibraryModal({appId: gameId}: {appId: string}): ReactElement {
	const {cards} = useCardsForGame({url: API_URL, logger: Logger, gameId});

    var ref = useRef();

    const height = 20;
    const [top, setTop] = useState<number>(210);

    useEffect(() => {
        if(!ref || !ref.current) return;
        const doc = (ref.current as unknown as HTMLElement).getRootNode() as Document;
        // const playButton = document.querySelector("[class^='appactionbutton_PlayButton']");

        const imageWindow = doc.querySelector("[class^='appdetails_Header']");
        const imageWindowBounds = imageWindow?.getBoundingClientRect();

        if(!imageWindowBounds)
            return;
        
        setTop(imageWindowBounds.height - height);
        Logger.Log("Set Top For Window bacuse of bounds", {imageWindowBounds});
    }, []);

    if(!cards)
    {
        //Logger.Error("Unable to find Card");
        return (<></>);
    }

    if(!cards.length)
    {
        Logger.Debug("No MicroSD card could be found for {appId}", {appId: gameId});
        return (<></>);
    }

    return (
        <div
            //@ts-ignore
            ref={ref}
            className="microsdeck-app-modal"
            style={{ position: 'absolute', height, top, left: '20px' }}
        >
            <FaSdCard size={20} />
            <span>
				{cards.map(v => v.name || UNAMED_CARD_NAME).join(", ")}
            </span>
        </div>
    )
}
