import { createContext, useContext, useEffect, useState } from "react";
import { MicroSDeck } from "../MicoSDeck.js";
import { CardAndGames, CardsAndGames } from "../types.js";

const MicroSDeckContext = createContext<MicroSDeckContext>(null as any);
export const useMicroSDeckContext = () => useContext(MicroSDeckContext);

interface ProviderProps {
	microSDeck: MicroSDeck
}

interface PublicMicroSDeck {
	currentCardAndGames: CardAndGames | undefined;
	cardsAndGames: CardsAndGames;
}

interface MicroSDeckContext extends PublicMicroSDeck {
	microSDeck: MicroSDeck
}

export function MicroSDeckContextProvider({ children, microSDeck }:  React.PropsWithChildren<ProviderProps>) {
	const [publicState, setPublicState] = useState<PublicMicroSDeck>({
		...microSDeck.getProps()
	});

	useEffect(() => {
		function onUpdate() {
			setPublicState({
				...microSDeck.getProps()
			});
		}

		microSDeck.eventBus.addEventListener("update", onUpdate);

		return () => {
			microSDeck.eventBus.removeEventListener("update", onUpdate);
		}
	}, []);

	return (
		<MicroSDeckContext.Provider
			value={{
				...publicState,
				microSDeck: microSDeck
			}}
		>
			{children}
		</MicroSDeckContext.Provider>
	)
}
