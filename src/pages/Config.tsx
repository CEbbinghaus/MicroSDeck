import { GetCardsAndGames } from "../hooks/backend";
import CardList from "../components/CardList";

export default function MicroSDeckConfigPage() {
    const { cards, refresh } = GetCardsAndGames();
  
    return (
      <div style={{ marginTop: 24, color: "white" }}>
        <h1 style={{marginLeft: 24, marginBottom: 5}}>MicroSDeck</h1>
        {!cards ? "Pending..." : <CardList cards={cards}/>}
      </div>
    );
  };