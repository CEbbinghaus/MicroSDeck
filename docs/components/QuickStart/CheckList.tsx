import React, { ReactElement } from "react";
import { FaCheckCircle, FaQuestionCircle } from "react-icons/fa";
import { useMicroSDeckContext, CardsAndGames } from "../../../lib/src";

type CheckListItem = {
	check: (cardsAndGames: CardsAndGames) => boolean,
	body: any
}

export function CheckListItem({ children, check }: React.PropsWithChildren<{ check: (cardsAndGames: CardsAndGames) => boolean }>): ReactElement {
	const { cardsAndGames } = useMicroSDeckContext();

	return (
		<>
			{check(cardsAndGames) ? <FaQuestionCircle /> : <FaCheckCircle />}
			{children}
		</>
	);
}

export function CheckList({ children }: React.PropsWithChildren<{}>): ReactElement {
	return (<>{children}</>);
}
