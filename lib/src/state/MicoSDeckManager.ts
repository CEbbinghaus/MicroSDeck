import { fetchCardsAndGames, fetchCardsForGame, fetchCurrentCardAndGames, fetchDeleteCard, fetchEventPoll, fetchHealth, fetchUpdateCard, fetchVersion } from "../backend.js";
import Logger from "lipe";
import { CardAndGames, CardsAndGames, MicroSDCard } from "../types.js"

function sleep(ms: number): Promise<void> {
	return new Promise(resolve => setTimeout(() => resolve(), ms));
}

export class MicroSDeckManager {
	private abortController = new AbortController();

	private logger: Logger | undefined;
	private fetchProps!: { url: string, logger?: Logger | undefined };

	public eventBus = new EventTarget();

	private enabled: boolean = false;
	public get Enabled() {
		return this.enabled;
	}
	private version: string | undefined;
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

		this.logger?.Log("Initializing MicroSDeckManager");

		this.fetchProps = props;

		this.fetch();
		this.subscribeToUpdates();
	}

	destruct() {
		if (this.isDestructed) return;
		this.isDestructed = true;
		this.logger?.Log("Deinitializing MicroSDeckManager");
		this.abortController.abort("destruct");
	}

	async fetch() {
		this.enabled = await fetchHealth(this.fetchProps);
		this.version = await fetchVersion(this.fetchProps);

		await this.fetchCurrent();
		await this.fetchCardsAndGames();
		this.eventBus.dispatchEvent(new Event("update"));
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
			version: this.version,
			cardsAndGames: this.cardsAndGames,
			currentCardAndGames: this.currentCardAndGames
		}
	}

	private async subscribeToUpdates() {
		let signal = this.abortController.signal;

		let sleepDelay = 500;
		this.logger?.Debug("Starting poll");

		while (true) {
			if (signal.aborted) {
				this.logger?.Debug("Aborting poll")
				return;
			}

			if (this.pollLock !== undefined) {
				this.logger?.Error("Tried Polling twice at the same time...");
				return;
			}

			this.pollLock = {};
			this.logger?.Debug("Poll");

			let result = await fetchEventPoll({ ...this.fetchProps, signal });

			this.logger?.Debug("Backend detected an update: {result}", { result });

			switch (result) {
				// Server is down. Lets try again but back off a bit
				case undefined:
					this.logger?.Warn("Unable to contact Server. Backing off and waiting {sleepDelay}ms", { sleepDelay });
					await sleep(sleepDelay = Math.min(sleepDelay * 1.5, 1000 * 60));
					break;

				// Request must have timed out
				case false:
					sleepDelay = 100;
					break;

				// We got an update. Time to refresh.
				default:
					this.eventBus.dispatchEvent(new Event(result));
					await this.fetch();
			}

			this.pollLock = undefined;
		}
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
