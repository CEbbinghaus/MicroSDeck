import { TextField } from "decky-frontend-lib";
import { FormEvent, FormEventHandler, Fragment, useRef } from "react";
import { SetNameForMicroSDCard } from "../hooks/backend";

export type CardProps = {
    games: Game[],
    card: MicroSDCard
}

export default function Card({ card, games }: CardProps) {
    const nameInputRef = useRef<any>();


    function Submit(event: FormEvent) {
        let value = nameInputRef.current?.element?.value;
        nameInputRef.current?.element?.blur();
        SetNameForMicroSDCard(card.uid, value);
        event.preventDefault();
    }

    return (
        <div style={{ width: "calc(30% - 32px)", height: "300px", overflow: "hidden", margin: "8px", padding: "8px", display: "inline-block", verticalAlign: "top", borderRadius: "12px", backgroundColor: "#3D4450" }}>
            <h6 style={{ margin: 0, padding: 0 }}>ID: {card.uid}</h6>

            <form onSubmit={Submit}>
                <TextField
                    // placeholder="Name"
                    // value={card.name}
                    placeholder={card.name}
                    //@ts-ignore
                    ref={nameInputRef}
                    bShowClearAction={true}
                />
            </form>
            <p style={{
                fontSize: "12px",
                margin: 0,
                overflow: "hidden",
                marginTop: "5px",
                display: "-webkit-box",
                WebkitLineClamp: 14,
                WebkitBoxOrient: "vertical",
                padding: 0,
                textOverflow: "ellipsis"
            }}>
                {games.map(game => <Fragment>{game.name}{<br />}</Fragment>)}
            </p>
        </div>
    );
}