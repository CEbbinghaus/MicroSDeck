export type MicroSDEntryType = {
	uid: string
}

export type MicroSDCard = {
	uid: string,
	name: string,
	games: string[],
	position: number,
	hidden: boolean,
}

export type Game = {
	uid: string,
	name: string,
	size: number,
	is_steam: boolean,
}

export type CardAndGames = [MicroSDCard, Game[]];

export type CardsAndGames = CardAndGames[];

export type FrontendSettings = {
	dismissed_docs: boolean	
}