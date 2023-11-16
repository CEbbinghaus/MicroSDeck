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

export type Event = {
	event: string,
	data?: string,
	id?: string
}

function decodeEvent(event: string): Event {
	console.log(`Recieved event to process: [${event}]`);

	var result = { event: "message" };
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

function eventDecodeStream() {
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
					controller.enqueue(decodeEvent(message));
				}
			}
		}
	})
}



function makeWriteableEventStream(eventTarget: EventTarget) {
	return new WritableStream<Event>({
		start() {
			eventTarget.dispatchEvent(new Event('start'))
		},
		write(message) {
			eventTarget.dispatchEvent(new CustomEvent(message.event, { detail: message }));
		},
		close() {
			eventTarget.dispatchEvent(new CloseEvent('close'));
		},
		abort(reason) {
			eventTarget.dispatchEvent(new CloseEvent('abort', { reason }));
		}
	})
}

export type EventCallback = (event: string, data: Event) => any;
function makeCallbackEventStream(callback: EventCallback) {
	return new WritableStream<Event>({
		start() { },
		write(message) {
			callback(message.event, message);
		},
	})
}

type EventTargetSink = { target: EventTarget };
type CallbackSink = { callback: EventCallback };

function isEventTargetSink(sink: EventTargetSink | CallbackSink): sink is EventTargetSink {
	return (sink as EventTargetSink).target !== undefined;
  }

function determineOutputSink(sink: EventTargetSink | CallbackSink) {
	if(isEventTargetSink(sink)) {
		return makeWriteableEventStream(sink.target);
	} else {
		return makeCallbackEventStream(sink.callback);
	}
}
export async function fetchEventTarget(props: FetchProps & (EventTargetSink | CallbackSink), init?: RequestInit) {
	const eventDecoder = eventDecodeStream()
	const outStream = determineOutputSink(props);
	await fetch(`${props.url}/listen`, {...init, keepalive: true})
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