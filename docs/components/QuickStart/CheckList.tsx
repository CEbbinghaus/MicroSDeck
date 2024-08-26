import React, { ReactElement } from "react";
import { FaCheckCircle, FaQuestionCircle } from "react-icons/fa";
import { useMicroSDeckContext, CardsAndGames } from "../../../lib/src";

type CheckListItem = {
	check: (cardsAndGames: CardsAndGames) => boolean,
	body: any
}

export function CheckListItem({ title, children, check }: React.PropsWithChildren<{ title: string, check: (cardsAndGames: CardsAndGames) => boolean }>): ReactElement {
	const { cardsAndGames } = useMicroSDeckContext();

	const is_completed = /* Math.random() < 0.5 && */ check(cardsAndGames);

	return (
		<>
			<div style={{display: "flex", flexDirection: "row"}}>
				<div style={{ marginTop: "1.6em", marginRight: "12px" }}>{is_completed ? <FaCheckCircle style={{ color: "#7df47d" }} size={26} /> : <FaQuestionCircle style={{ color: "#7dbcf4" }} size={26} />}</div>
				<h2 style={{marginBottom: "0px"}}>{title}</h2>
			</div>
			{is_completed ? <></> : children}
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
