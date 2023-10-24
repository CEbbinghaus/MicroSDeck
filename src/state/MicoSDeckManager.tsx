import { Logger } from "../Logging";
import { fetchCardsAndGames, fetchCurrentCardAndGames, fetchDeleteCard, fetchUpdateCard } from "../hooks/backend";
import { CardAndGames, CardsAndGames, MicroSDCard } from "../lib/Types"

export class MicroSDeckManager {
	public eventBus = new EventTarget();

	private currentCardAndGames: CardAndGames | undefined; 
	private cardsAndGames: CardsAndGames = [];


	init() {
		this.fetch();
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

	async updateCard(card: MicroSDCard) {
		Logger.Debug("Updating card {uid}", card);
		await fetchUpdateCard(card);
		await this.fetch()
	}
	
	async deleteCard(card: MicroSDCard) {
		Logger.Log("Deleting Card {uid}", card);
		await fetchDeleteCard(card);
		await this.fetch();
	}
	
	async hideCard(card: MicroSDCard) {
		card.hidden = true;
		//TODO: Implement
		Logger.Log("Card {uid} was supposed to be hidden", card);
	}
}
