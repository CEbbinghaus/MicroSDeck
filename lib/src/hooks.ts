import { useEffect, useState } from "react";
import { MicroSDCard } from "./types";
import { FetchProps, fetchCardsForGame } from "./backend";

export function useCardsForGame(props: FetchProps & { gameId: string }) {

	const [value, setValue] = useState<MicroSDCard[] | undefined>()

	async function refresh() {
		setValue(await fetchCardsForGame(props))
	}

	useEffect(() => {
		(async () => {
			await refresh();
		})();
	}, [props.gameId]);
	
	return {
		cards: value,
		refresh
	}
}