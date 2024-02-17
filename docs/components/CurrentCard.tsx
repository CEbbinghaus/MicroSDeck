import { ReactElement } from "react";
import { useMicroSDeckContext } from "../../lib/src";
import React from "react";

export function CurrentCard(): ReactElement {
	const { currentCardAndGames } = useMicroSDeckContext();

	if (!currentCardAndGames) {
		return (<>Unfortunately, This little trick only works while you have a MicroSD card inserted. Try sticking one in</>);
	}

	const [card, games] = currentCardAndGames;

	return (
		<>
			You current have card "<b>{card.name}</b>" inserted.<br />
			But did you know it also has a unique ID?<br />
			Here it is: <b>{card.uid}</b>
			<br />
			<br />
			{
				games.length ?
					(<>
						For good measure, Here are all the game Id's too:<br />
						{
							games.map(v => (
								<>
									<b>{v.name}:</b> {v.uid}<br />
								</>
							))
						}
					</>) :
					(<>
						There are currently no games installed on this MicroSD card. If there were their Id's would be printed here.
					</>)
			}
			<br />
			<br />
			Pretty cool ey? It'll be even more impressive after you remove the MicroSD card you have currently plugged in 😉
		</>
	)
}