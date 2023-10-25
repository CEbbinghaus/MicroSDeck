import { Logger } from "../Logging";
import { fetchCardsAndGames, fetchCurrentCardAndGames, fetchDeleteCard, fetchEventPoll, fetchUpdateCard } from "../hooks/backend";
import { CardAndGames, CardsAndGames, MicroSDCard } from "../lib/Types"

function sleep(ms: number): Promise<void> {
	return new Promise(resolve => setTimeout(() => resolve(), ms));
}

export class MicroSDeckManager {
	private abortController = new AbortController();

	public eventBus = new EventTarget();

	private currentCardAndGames: CardAndGames | undefined;
	private cardsAndGames: CardsAndGames = [];


	private pollLock: any | undefined;

	init() {
		Logger.Log("Initializing MicroSDeckManager");
		this.init = () => { throw "Do Not call init more than once"; };
		this.fetch();
		this.subscribeToUpdates();
	}

	deinit() {
		Logger.Log("Deinitializing MicroSDeckManager");
		this.abortController.abort("deinit");
	}

	async fetch() {
		this.currentCardAndGames = await fetchCurrentCardAndGames();
		this.cardsAndGames = await fetchCardsAndGames() || [];
		this.eventBus.dispatchEvent(new Event("stateUpdate"));
	}

	getCardsAndGames() {
		return {
			cardsAndGames: this.cardsAndGames
		}
	}

	getCurrentCard() {
		return {
			currentCardAndGames: this.currentCardAndGames
		}
	}

	async subscribeToUpdates() {
		let signal = this.abortController.signal;

		let sleepDelay = 500;
		Logger.Debug("Starting poll");

		while (true) {
			if(signal.aborted) {
				Logger.Debug("Aborting poll")
				return;
			}

			if (this.pollLock !== undefined) {
				Logger.Error("Tried Polling twice at the same time...");
				return;
			}
			
			this.pollLock = {};
			Logger.Debug("Poll");

			let result = await fetchEventPoll({signal});

			Logger.Debug("Result was: " + (result === undefined ? "undefined" : result) , {result});

			switch(result) {
				// Server is down. Lets try again but back off a bit
				case undefined:
					Logger.Warn("Unable to contact Server. Backing off and waiting {sleepDelay}ms", {sleepDelay});
					await sleep(sleepDelay *= 1.5);
				break;
				
				// We got an update. Time to refresh.
				case true:
					Logger.Debug("Card detected an update.");
					await this.fetch();

				// Request must have timed out
				case false:
					sleepDelay = 100;
					break;
			}

			this.pollLock = undefined;
		}
	}

	async updateCard(card: MicroSDCard) {
		Logger.Debug("Updating card {uid}", card);
		await fetchUpdateCard(card);
		await this.fetch()
	}

	async deleteCard(card: MicroSDCard) {
		Logger.Debug("Deleting Card {uid}", card);
		await fetchDeleteCard(card);
		await this.fetch();
	}

	async hideCard(card: MicroSDCard) {
		card.hidden = true;
		//TODO: Implement
		Logger.Log("Card {uid} was supposed to be hidden", card);
	}
}
