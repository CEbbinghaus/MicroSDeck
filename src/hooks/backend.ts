import { useEffect, useState } from 'react'
import { call_backend } from 'usdpl-front';

export function getValue(appId: string | undefined){
    const [value, setValue] = useState<any>(false)

    async function refresh() {
        const result = await call_backend("ping", []);

        setValue(result)
    }

    useEffect(() => {
        (async () => {
            const result = await call_backend("ping", []);
            setValue(result)
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
        const result = await call_backend("list_cards_with_games", []) as any[];

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
