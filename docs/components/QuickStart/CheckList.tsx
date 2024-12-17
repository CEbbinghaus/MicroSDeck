import React, { ReactElement } from "react";
import { FaCheckCircle, FaQuestionCircle } from "react-icons/fa";
import { useMicroSDeckContext, CardsAndGames } from "../../../lib/src";

type CheckListItem = {
	check: (cardsAndGames: CardsAndGames) => boolean,
	body: any
}

export function CheckListItem({ title, children, check }: React.PropsWithChildren<{ title: string, check: (cardsAndGames: CardsAndGames) => boolean }>): ReactElement {
	const { cardsAndGames } = useMicroSDeckContext();

	const is_completed = false && check(cardsAndGames);

	return (
		<>
			<div style={{display: "inline-block"}}>
				{is_completed ? <FaCheckCircle style={{ color: "#0f0" }} size={32} /> : <FaQuestionCircle style={{ color: "#00f	" }} size={32} />}
				<h2>{title}</h2>
			</div>
			{is_completed ? <></> : children}
			<br/>
		</>
	);
}

export function CheckList({ children }: React.PropsWithChildren<{}>): ReactElement {
	return (
		<>
			{children}
		</>
	);
}
