import { useEffect, useState } from 'react'
import { call_backend } from 'usdpl-front';
import { Logger } from '../Logging';

export async function SetNameForMicroSDCard(CardId: string, Name: string){
    await call_backend("set_name_for_card", [CardId, Name])
}


export function GetCardsForGame(appId: string){
    const [value, setValue] = useState<string | MicroSDCard[] | undefined>()

    async function refresh() {
        const result = await call_backend("get_card_for_game", [appId]);
        setValue(result[0])
    }

    useEffect(() => {
        (async () => {
            const result = await call_backend("get_card_for_game", [appId]);
            setValue(result[0])
        })();
      }, [appId])

    return {
        value,
        refresh
    }
}

export type CardsAndGames = [MicroSDCard, Game[]][];
export function GetCardsAndGames() {
    const [cards, setValue] = useState<CardsAndGames | null>(null)

    async function runQuery() {
        Logger.Log("Running Query");
        const result = await call_backend("list_cards_with_games", []) as any[];

        Logger.Log("Query Finished", {result});

        if(typeof result[0] === "string")
            Logger.Log("Unable to retrieve data", {result});
        else {
            setValue(result[0])
        }
    }

    useEffect(() => {
        (async () => {
            await runQuery();
        })();
      }, [])

    return {
        cards,
        refresh: runQuery
    }
}
