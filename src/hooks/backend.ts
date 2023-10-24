import { useEffect, useState } from 'react'
import { Logger } from '../Logging';
import { API_URL } from '../const';
import { CardAndGames, CardsAndGames, MicroSDCard } from '../lib/Types';

export async function SetNameForMicroSDCard(CardId: string, Name: string) {
	await fetch(`${API_URL}/SetNameForCard`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({ id: CardId, name: Name }),
		referrerPolicy: "unsafe-url",
	})
	.catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));
}

export function GetCardsForGame(appId: string) {
	const [value, setValue] = useState<string | MicroSDCard[] | undefined>()

	async function refresh() {
		const result = await fetch(`${API_URL}/GetCardsForGame/${appId}`, { referrerPolicy: "unsafe-url", })
			.then(res => res.json())
			.catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));

		setValue(result)
	}

	useEffect(() => {
		(async () => {
			await refresh();
		})();
	}, [appId])

	return {
		value,
		refresh
	}
}

export async function fetchDeleteCard(card: MicroSDCard) {
	await fetch(`${API_URL}/card/${card.uid}`, {
		method: "DELETE",
		referrerPolicy: "unsafe-url",
	})
	.catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));
}

export async function fetchUpdateCard(card: MicroSDCard) {
	await fetch(`${API_URL}/card/${card.uid}`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(card),
		referrerPolicy: "unsafe-url",
	})
	.catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));
}

export async function fetchCurrentCardAndGames(): Promise<CardAndGames | undefined> {
	return await fetch(`${API_URL}/current`, { referrerPolicy: "unsafe-url", })
		.then(res => res.json())
		.catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));
}
export async function fetchCardsAndGames(): Promise<CardsAndGames | undefined> {
	return await fetch(`${API_URL}/ListCardsWithGames`, { referrerPolicy: "unsafe-url", })
		.then(res => res.json())
		.catch(Error => Logger.Error("There was a critical error: \"{Error}\"", { Error }));
}

export function GetCardsAndGames() {
	const [cards, setValue] = useState<CardsAndGames | null>(null)

	async function runQuery() {
		const result = await fetchCardsAndGames();
		setValue(result || null)
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
