import { Event as BackendEvent, EventType, fetchCardsAndGames, fetchCardsForGame, fetchCurrentCardAndGames, fetchDeleteCard, fetchEventTarget, fetchHealth, fetchUpdateCard, fetchVersion } from "./backend.js";
import Logger from "lipe";
import { CardAndGames, CardsAndGames, MicroSDCard } from "./types.js"

import semverParse from "semver/functions/parse"
import semverEq from "semver/functions/eq.js"

//@ts-ignore TS6059 The relative path will stay the same
import { version } from "../package.json"

function sleep(ms: number): Promise<void> {
	return new Promise(resolve => setTimeout(() => resolve(), ms));
}

interface EventBusListener<T extends Event> {
	(evt: T): void;
}

interface EventBusListenerObject<T extends Event> {
	handleEvent(object: T): void;
}

export type EventBus<K extends string, T extends Event> = {
	addEventListener(type: K, callback: EventBusListener<T> | EventBusListenerObject<T> | null, options?: boolean | AddEventListenerOptions | undefined): void;
	dispatchEvent(event: T): boolean;
	removeEventListener(type: K, callback: EventBusListener<T> | EventBusListenerObject<T> | null, options?: boolean | EventListenerOptions | undefined): void;
}

export { version };

/**
 * Matches the current MicroSDeck version against the version provided.
 * Returns true if the semver matches and compatibility is guaranteed. 
 * @export
 * @param {string} version1 This should be in valid semver notation of x.y.z
 * @param {string} version2 This should be in valid semver notation of x.y.z
 */
export function IsMatchingSemver(version1: string, version2: string, strict: boolean = false): boolean {
	const semVersion1 = semverParse(version1) ?? (() => {throw new Error("Current Version is not a valid Semver. (How did we get here?)")})();
	const semVersion2 = semverParse(version2) ?? (() => {throw new Error("provided Version is not a valid Semver.")})();;

	// the two major versions do not match. Not valid
	if(semVersion1.major != semVersion2.major && semVersion1.major != 0) {
		return false;
	}

	// The minor version can introduce breaking changes while <1.0
	if(semVersion1.minor != semVersion2.minor &&  semVersion1.major == 0) {
		return false;
	}

	if(!strict) {
		return true;
	}

	return semverEq(semVersion1, semVersion2);
}

export class MicroSDeck {
	private abortController = new AbortController();

	private logger: Logger | undefined;
	private fetchProps!: { url: string, logger?: Logger | undefined };

	public eventBus = new EventTarget() as unknown as EventBus<EventType, Event | CustomEvent<BackendEvent>>;

	public get Version() {
		return version;
	}

	private enabled: boolean = false;
	public get Enabled() {
		return this.enabled;
	}
	
	private backendVersion: string | undefined;
	public get BackendVersion() {
		return this.backendVersion;
	}

	private currentCardAndGames: CardAndGames | undefined;
	public get CurrentCardAndGames() {
		return this.currentCardAndGames;
	}
	private cardsAndGames: CardsAndGames = [];
	public get CardsAndGames() {
		return this.cardsAndGames;
	}

	private pollLock: any | undefined;

	private isDestructed = false;

	constructor(props: { logger?: Logger, url: string }) {
		this.logger = props.logger;

		this.eventBus.addEventListener

		this.logger?.Log("Initializing MicroSDeck");

		this.fetchProps = props;

		this.fetch();
		this.subscribeToUpdates();
	}

	destruct() {
		this.logger?.Debug("Deconstruct Called");
		if (this.isDestructed) return;
		this.logger?.Log("Deinitializing MicroSDeck");
		this.abortController.abort("destruct");
		this.isDestructed = true;
	}

	async fetch() {
		if(!await fetchHealth(this.fetchProps)) {
			this.enabled = false;
			this.logger?.Log("Fetch failed. Server is offline...");
			return;
		}
		this.backendVersion = await fetchVersion(this.fetchProps);

		await this.fetchCurrent();
		await this.fetchCardsAndGames();
		this.eventBus.dispatchEvent(new Event("update"));
		this.enabled = true;
	}

	async fetchCurrent() {
		this.currentCardAndGames = await fetchCurrentCardAndGames(this.fetchProps);
	}
	async fetchCardsAndGames() {
		this.cardsAndGames = await fetchCardsAndGames(this.fetchProps) || [];
	}

	getProps() {
		return {
			enabled: this.enabled,
			version: this.backendVersion,
			cardsAndGames: this.cardsAndGames,
			currentCardAndGames: this.currentCardAndGames
		}
	}

	private async subscribeToUpdates() {
		let signal = this.abortController.signal;

		let sleepDelay = 5000;

		if (this.pollLock !== undefined) {
			this.logger?.Error("Tried Polling twice at the same time...");
			return;
		}

		this.pollLock = {};

		this.logger?.Debug("Starting poll");

		try {
			while (true) {
				if (signal.aborted) {
					this.logger?.Debug("Aborting poll")
					return;
				}

				this.logger?.Debug("Poll listen");

				await new Promise(async (res) => {
					await sleep(sleepDelay);

					fetchEventTarget({ ...this.fetchProps, callback: this.handleCallback.bind(this) }, { signal })
						.catch((reason) => {
							this.logger?.Warn(`Listen was aborted with reason "${reason}"`);
							res(0);
						});
				})
			}
		} finally {
			this.pollLock = undefined;
		}

	}

	async handleCallback(event: EventType, data?: BackendEvent) {
		await this.fetch();
		this.eventBus.dispatchEvent(new CustomEvent(event, { detail: data }));
	}

	async updateCard(card: MicroSDCard) {
		this.logger?.Debug("Updating card {uid}", card);
		await fetchUpdateCard({ ...this.fetchProps, card });
		await this.fetch()
	}

	async deleteCard(card: MicroSDCard) {
		this.logger?.Debug("Deleting Card {uid}", card);
		await fetchDeleteCard({ ...this.fetchProps, card });
		await this.fetch();
	}

	async hideCard(card: MicroSDCard) {
		card.hidden = true;
		//TODO: Implement
		this.logger?.Log("Card {uid} was supposed to be hidden", card);
	}

	async fetchCardsForGame(gameId: string) {
		return await fetchCardsForGame({ ...this.fetchProps, gameId })
	}
}
