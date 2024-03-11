import React, { ReactElement } from "react";
import { FaCheckCircle, FaQuestionCircle } from "react-icons/fa";
import { useMicroSDeckContext, CardsAndGames } from "../../../lib/src";

type CheckListItem = {
	check: (cardsAndGames: CardsAndGames) => boolean,
	body: any
}

export function CheckList({ items }: { items: CheckListItem[] }): ReactElement {
	const { cardsAndGames } = useMicroSDeckContext();

	const item_completions = items.map(v => { return { body: v.body, is_completed: v.check(cardsAndGames) } });

	return (
		<>
			{
				item_completions.map(v =>
					<>
						{v.is_completed ? <FaQuestionCircle/> : <FaCheckCircle/>}
						{v.body}
					</>
				)
			}
		</>
	)
}
