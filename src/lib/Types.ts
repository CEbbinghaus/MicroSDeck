export type MicroSDEntryType = {
	uid: string
}

export type MicroSDCard = {
	uid: string,
	name: string,
	games: string[],
	position: number | undefined,
	hidden: boolean | undefined
}


export type Game = {
	uid: string,
	name: string,
	size: number,
	card: string
}

export type CardAndGames = [MicroSDCard, Game[]];

export type CardsAndGames = CardAndGames[];