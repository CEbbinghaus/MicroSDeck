import { Focusable } from "decky-frontend-lib";
import { CardsAndGames } from "../hooks/backend";
import Card from "./Card"

export default function CardList({ cards }: { cards: CardsAndGames }) {

    if(!cards?.length) {
        return (<h1>No Cards have been registered yet...</h1>)
    }

    return (
        <Focusable style={{ display: "flex", overflow: "scroll" }}>
            {
                cards.map(([card, games]) => (
                    <Card card={card} games={games} />
                ))
            }
        </Focusable>
    );
}