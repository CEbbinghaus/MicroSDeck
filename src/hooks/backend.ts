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

export async function fetchEventPoll({signal}: {signal: AbortSignal}): Promise<boolean | undefined> {
	try {
		const response = await fetch(`${API_URL}/listen`, {
			keepalive: true,
			referrerPolicy: "unsafe-url",
			signal
		});

		if (response.ok) {
			return true;
		}

		Logger.Log("Poll timed out...")
		return false;
	} catch (error) {
		Logger.Error("Lost contact with server..");
		return undefined;
	}
}

export async function fetchDeleteCard(card: MicroSDCard) {
	try {
		await fetch(`${API_URL}/card/${card.uid}`, {
			method: "DELETE",
			referrerPolicy: "unsafe-url",
		});
	} catch (error) {
		Logger.Error("There was a critical error: \"{Error}\"", { Error });
	}
}

export async function fetchUpdateCard(card: MicroSDCard) {
	try {
		await fetch(`${API_URL}/card/${card.uid}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(card),
			referrerPolicy: "unsafe-url",
		});
	} catch (error) {
		Logger.Error("There was a critical error: \"{Error}\"", { Error });
	}
}

export async function fetchCurrentCardAndGames(): Promise<CardAndGames | undefined> {
	try {
		let result = await fetch(`${API_URL}/current`, { referrerPolicy: "unsafe-url", });
		
		if(!result.ok) {
			Logger.Warn("Fetch returned non 200 code {status} status, {statusText}", {status: result.status, statusText: result.statusText})
			return undefined;
		}

		return await result.json();
	} catch (error) {
		Logger.Error("There was a critical error: \"{Error}\"", { Error });
		return undefined;
	}
}
export async function fetchCardsAndGames(): Promise<CardsAndGames | undefined> {
	try {
		let result = await fetch(`${API_URL}/ListCardsWithGames`, { referrerPolicy: "unsafe-url", });
		
		if(!result.ok) {
			Logger.Warn("Fetch returned non 200 code {status} status, {statusText}", {status: result.status, statusText: result.statusText})
			return undefined;
		}

		return await result.json();
	} catch (error) {
		Logger.Error("There was a critical error: \"{Error}\"", { Error });
		return undefined;
	}
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
