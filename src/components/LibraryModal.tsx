// import { ServerAPI } from 'decky-frontend-lib'
import React, { ReactElement, useEffect, useRef, useState} from 'react'
import { FaSdCard } from 'react-icons/fa'
import { GetCardsForGame } from '../hooks/backend';
import { Logger } from '../Logging';
import { UNAMED_CARD_NAME } from '../const';

export default function LibraryModal({appId}: {appId: string}): ReactElement {
    var ref = useRef();

    const height = 20;
    const [top, setTop] = useState<number>(210);

    const {value} = GetCardsForGame(appId);

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
    });

    if(!value)
    {
        //Logger.Error("Unable to find Card");
        return (<></>);
    }

    if(typeof value === "string")
    {
        Logger.Error("Error retrieving SD Card: \"{error}\"", {error: value})
        return (<></>);
    }

    if(!value.length)
    {
        Logger.Info("No MicroSD card could be found for {appId}", {appId});
        return (<></>);
    }

    // if (!data) return (<></>);

    return (
        <div
            //@ts-ignore
            ref={ref}
            className="microsdeck-app-modal"
            style={{ position: 'absolute', height, top, left: '20px' }}
        >
            <FaSdCard size={20} />
            <span>
				{value.map(v => v.name || UNAMED_CARD_NAME).join(", ")}
            </span>
        </div>
    )
}
