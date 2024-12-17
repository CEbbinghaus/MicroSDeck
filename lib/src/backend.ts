import Logger from 'lipe';
import { CardAndGames, CardsAndGames, Game, MicroSDCard } from "./types.js";

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
	[key: string]: string | undefined,
	event: EventType,
	data?: string,
	id?: string
}

function decodeEvent(message: string, logger?: Logger): Event {
	logger?.Debug(`Received event to process: [{message}]`, { message });

	var result: Event = { event: "message" };

	for (let line of message.split('\n')) {
		let [key, value] = line.split(":").map(v => v.trim());
		if (!key) {
			throw new Error("No key was present for event " + message);
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
				// extract the full message out of the buffer
				const message = buffer.substring(0, pos).trim();

				// Remove the message from the buffer and reset the index
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

export type SettingNames = 
	"*" |
	"backend" |
	"backend:port" |
	"backend:scan_interval" |
	"backend:store_file" |
	"backend:log_file" |
	"backend:log_level" |
	"backend:startup" |
	"backend:startup:skip_validate" |
	"backend:startup:skip_clean" |
	"frontend" |
	"frontend:dismissed_docs";

export async function fetchGetSetting({ url, logger, setting_name }: FetchProps & { setting_name: SettingNames }): Promise<any | undefined> {
	const result = await wrapFetch({ url: `${url}/setting/${setting_name}`, logger });
	return result && JSON.parse(result) || result;
}

export async function fetchSetSetting({ url, logger, value, setting_name }: FetchProps & { setting_name: SettingNames, value: any }) {
	await wrapFetch({ url: `${url}/setting/${setting_name}`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify(value),
	});
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

export async function fetchCreateGame({ url, logger, game}: FetchProps & { game: Game }) {
	await wrapFetch({ url: `${url}/game/${game.uid}`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify(game),
	});
}

export async function fetchLinkCardAndGame({ url, logger, card_id, game_id}: FetchProps & { card_id: string, game_id: string }) {
	await wrapFetch({ url: `${url}/link`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify({card_id, game_id}),
	});
}

export async function fetchLinkCardAndManyGames({ url, logger, card_id, game_ids}: FetchProps & { card_id: string, game_ids: string[] }) {
	await wrapFetch({ url: `${url}/linkmany`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify({card_id, game_ids}),
	});
}

export async function fetchUnlinkCardAndGame({ url, logger, card_id, game_id}: FetchProps & { card_id: string, game_id: string }) {
	await wrapFetch({ url: `${url}/unlink`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify({card_id, game_id}),
	});
}

export async function fetchUnlinkCardAndManyGames({ url, logger, card_id, game_ids}: FetchProps & { card_id: string, game_ids: string[] }) {
	await wrapFetch({ url: `${url}/unlinkmany`, logger }, {
		method: "POST",
		...ApplicationJsonHeaders,
		body: JSON.stringify({card_id, game_ids}),
	});
}
