import { createContext, FC, useContext, useEffect, useState } from "react";
import { MicroSDeckManager } from "./MicoSDeckManager";
import { CardAndGames, CardsAndGames } from "../lib/Types";

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

export const MicroSDeckContextProvider: FC<ProviderProps> = ({ children, microSDeckManager }) => {
	const [publicState, setPublicState] = useState<PublicMicroSDeckManager>({
		...microSDeckManager.getCardsAndGames(),
		...microSDeckManager.getCurrentCard()
	});

	useEffect(() => {
		function onUpdate() {
			setPublicState({
				...microSDeckManager.getCardsAndGames(),
				...microSDeckManager.getCurrentCard()
			});
		}

		microSDeckManager.eventBus.addEventListener("stateUpdate", onUpdate);

		return () => {
			microSDeckManager.eventBus.removeEventListener("stateUpdate", onUpdate);
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
