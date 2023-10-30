import Logger from 'lipe';
import { CardAndGames, CardsAndGames, MicroSDCard } from "./types.js";

export type FetchProps = {
	url: string,
	logger?: Logger | undefined;
}

const ApplicationJsonHeaders = {
	headers: {
		"Content-Type": "application/json",
	}
}

async function wrapFetch({ url, logger }: FetchProps, init?: RequestInit): Promise<any | undefined> {
	try {
		const response = await fetch(url, init);

		if (!response.ok) {
			logger?.Debug("Fetching {url} returned a non 200 status code {status}. Response: {statusText}", response as any);
			return undefined;
		}

		if (response.headers.get("content-type") === "application/json")
			return await response.json();
		else
			return await response.text();

	} catch (err) {
		logger?.Error("Failed to fetch \"{url}\" with error {err}", { url, err });
	}
	return undefined;
}

export async function fetchEventPoll({ url, logger, signal }: FetchProps & { signal: AbortSignal }): Promise<string | boolean | undefined> {
	try {
		const result = await fetch(`${url}/listen`, {
			keepalive: true,
			signal
		});

		if (!result.ok) {
			logger?.Log("Poll timed out...");
			return false;
		}

		return await result.json();

	} catch (err) {
		logger?.Error("Fetch failed with error {err}", { err });
	}
	return undefined;
}

export async function fetchHealth({url, logger}: FetchProps): Promise<boolean> {
	return await wrapFetch({url: `${url}/health`, logger}) !== undefined;
}

export async function fetchVersion({url, logger}: FetchProps): Promise<string | undefined> {
	return await wrapFetch({url: `${url}/health`, logger});
}

export async function fetchDeleteCard({ url, logger, card }: FetchProps & { card: MicroSDCard }) {
	await wrapFetch({ url: `${url}/card/${card.uid}`, logger }, { method: "DELETE" });
}

export async function fetchUpdateCard({ url, logger, card }: FetchProps & { card: MicroSDCard }) {
	await wrapFetch({ url: `${url}/card/${card.uid}`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify(card),
	});
}

export async function fetchCurrentCardAndGames({url, logger}: FetchProps): Promise<CardAndGames | undefined> {
	return await wrapFetch({url: `${url}/current`, logger});
}

export async function fetchCardsAndGames({url, logger}: FetchProps): Promise<CardsAndGames | undefined> {
	return await wrapFetch({url: `${url}/list`, logger});
}

export async function fetchCardsForGame({ url, logger, gameId }: FetchProps & { gameId: string }): Promise<MicroSDCard[] | undefined> {
	return await wrapFetch({ url: `${url}/list/cards/${gameId}`, logger });
}