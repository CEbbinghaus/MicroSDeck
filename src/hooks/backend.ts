import { useEffect, useState } from 'react'
import { call_backend } from 'usdpl-front';

export async function SetNameForMicroSDCard(CardId: string, Name: string){
    await call_backend("set_name_for_card", [CardId, Name])
}


export function GetCardForGame(appId: string){
    const [value, setValue] = useState<[string | MicroSDCard] | undefined>()

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
    const [value, setValue] = useState<CardsAndGames | null>(null)

    async function runQuery() {
        console.log("Running Query");
        const result = await call_backend("list_cards_with_games", []) as any[];

        console.log("Query Finished", result);

        if(typeof result[0] === "string")
            console.error("Unable to retrieve data", result);
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
        value,
        refresh: runQuery
    }
}
