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

export type EventType = "start" | "close" | "abort" | "message" | "insert" | "remove" | "update" | "change";
export type Event = {
	event: EventType,
	data?: string,
	id?: string
}

function decodeEvent(event: string, logger?: Logger): Event {
	logger?.Debug(`Recieved event to process: [{event}]`, {event});

	var result = { event: "message" as EventType };
	var lines = event.split('\n');

	for (let line of lines) {
		let [key, value] = line.split(":").map(v => v.trim());
		if (!key) {
			throw new Error("No key was present for event " + event);
		}

		result[key] = value;
	}

	return result;
}

function decodeStreamEvents(logger?: Logger) {
	let buffer = "";
	let pos = 0;
	return new TransformStream<string, Event>({
		start() { },
		transform(chunk, controller) {
			buffer += chunk;
			while (pos < buffer.length) {
				if (buffer[pos] + buffer[pos + 1] != '\n\n') {
					++pos
					continue;
				}
				const message = buffer.substring(0, pos).trim();
				buffer = buffer.substring(pos + 2);
				pos = 0;

				if (message) {
					controller.enqueue(decodeEvent(message, logger));
				}
			}
		}
	})
}

export type EventCallback = (event: EventType, data?: Event) => any;
function makeCallbackEventStream(callback: EventCallback) {
	return new WritableStream<Event>({
		start() {
			callback("start");
		},
		write(message) {
			callback(message.event, message);
			callback("change");
		},
		close() {
			callback("close");
		},
		abort() {
			callback("abort");
		}
	})
}

export async function fetchEventTarget({url, logger, callback}: FetchProps & { callback: EventCallback }, init?: RequestInit) {
	const eventDecoder = decodeStreamEvents(logger);
	const outStream = makeCallbackEventStream(callback);

	await fetch(`${url}/listen`, {...init, keepalive: true})
		.then(response => {
			response.body?.pipeThrough(new TextDecoderStream())
				.pipeThrough(eventDecoder)
				.pipeTo(outStream);
		})
}

export async function fetchHealth({ url, logger }: FetchProps): Promise<boolean> {
	return await wrapFetch({ url: `${url}/health`, logger }) !== undefined;
}

export async function fetchVersion({ url, logger }: FetchProps): Promise<string | undefined> {
	return await wrapFetch({ url: `${url}/health`, logger });
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

export async function fetchUpdateCards({ url, logger, cards }: FetchProps & { cards: MicroSDCard[] }) {
	await wrapFetch({ url: `${url}/cards`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify(cards),
	});
}

export async function fetchCurrentCardAndGames({ url, logger }: FetchProps): Promise<CardAndGames | undefined> {
	return await wrapFetch({ url: `${url}/current`, logger });
}

export async function fetchCardsAndGames({ url, logger }: FetchProps): Promise<CardsAndGames | undefined> {
	return await wrapFetch({ url: `${url}/list`, logger });
}

export async function fetchCardsForGame({ url, logger, gameId }: FetchProps & { gameId: string }): Promise<MicroSDCard[] | undefined> {
	return await wrapFetch({ url: `${url}/list/cards/${gameId}`, logger });
}