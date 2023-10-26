import Logger from 'lipe';
import { CardAndGames, CardsAndGames, MicroSDCard } from "./types.js";

export type FetchProps = {
	url: string,
	logger?: Logger.default | undefined;
}

const ApplicationJsonHeaders = {
	headers: {
		"Content-Type": "application/json",
	}
}

async function wrapFetch({ url, logger }: FetchProps, init?: RequestInit): Promise<any | undefined> {
	try {
		const result = await fetch(url, { ...init, referrerPolicy: "unsafe-url" });

		if (!result.ok) {
			logger?.Debug("Fetching {url} returned a non 200 status code {status}. Response: {statusText}", result as any);
			return undefined;
		}

		if (init?.headers?.["Content-Type"])
			return await result.json();
		else
			return await result.text();

	} catch (err) {
		logger?.Error("Fetch failed with error {err}", { err });
	}
	return undefined;
}

export async function fetchEventPoll({ url, logger, signal }: FetchProps & { signal: AbortSignal }): Promise<boolean | undefined> {
	try {
		const result = await fetch(`${url}/listen`, {
			referrerPolicy: "unsafe-url",
			keepalive: true,
			signal
		});

		if (!result.ok) {
			logger?.Log("Poll timed out...");
			return false;
		}

		return true;

	} catch (err) {
		logger?.Error("Fetch failed with error {err}", { err });
	}
	return undefined;
}

export async function fetchHealth({url, logger}: FetchProps): Promise<boolean> {
	return await wrapFetch({url: `${url}/health`, logger}, ApplicationJsonHeaders) !== undefined;
}

export async function fetchVersion({url, logger}: FetchProps): Promise<string | undefined> {
	return await wrapFetch({url: `${url}/health`, logger}, ApplicationJsonHeaders);
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
	return await wrapFetch({url: `${url}/current`, logger}, ApplicationJsonHeaders);
}

export async function fetchCardsAndGames({url, logger}: FetchProps): Promise<CardsAndGames | undefined> {
	return await wrapFetch({url: `${url}/list`, logger}, ApplicationJsonHeaders);
}

export async function fetchCardsForGame({ url, logger, gameId }: FetchProps & { gameId: string }): Promise<MicroSDCard[] | undefined> {
	return await wrapFetch({ url: `${url}/list/cards/${gameId}`, logger });
}