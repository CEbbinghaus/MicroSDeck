import { createContext, useContext, useEffect, useState } from "react";
import { MicroSDeckManager } from "../MicoSDeckManager.js";
import { CardAndGames, CardsAndGames } from "../types.js";

const MicroSDeckContext = createContext<MicroSDeckContext>(null as any);
export const useMicroSDeckContext = () => useContext(MicroSDeckContext);

interface ProviderProps {
	microSDeckManager: MicroSDeckManager
}

interface PublicMicroSDeckManager {
	currentCardAndGames: CardAndGames | undefined;
	cardsAndGames: CardsAndGames;
}

interface MicroSDeckContext extends PublicMicroSDeckManager {
	microSDeckManager: MicroSDeckManager
}

export function MicroSDeckContextProvider({ children, microSDeckManager }:  React.PropsWithChildren<ProviderProps>) {
	const [publicState, setPublicState] = useState<PublicMicroSDeckManager>({
		...microSDeckManager.getProps()
	});

	useEffect(() => {
		function onUpdate() {
			setPublicState({
				...microSDeckManager.getProps()
			});
		}

		microSDeckManager.eventBus.addEventListener("update", onUpdate);

		return () => {
			microSDeckManager.eventBus.removeEventListener("update", onUpdate);
		}
	}, []);

	return (
		<MicroSDeckContext.Provider
			value={{
				...publicState,
				microSDeckManager
			}}
		>
			{children}
		</MicroSDeckContext.Provider>
	)
}
