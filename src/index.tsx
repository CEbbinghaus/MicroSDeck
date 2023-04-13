import {
  ButtonItem,
  definePlugin,
  DialogButton,
  DropdownItem,
  Navigation,
  PanelSection,
  PanelSectionRow,
  Router,
  ServerAPI,
  SideMenu,
  sleep,
  staticClasses,
} from "decky-frontend-lib";
import { VFC } from "react";
import { FaSdCard } from "react-icons/fa";

import logo from "../assets/logo.png";

import { init_usdpl, target_usdpl, init_embedded, call_backend } from "usdpl-front";
import { CardsAndGames, GetCardsAndGames } from "./hooks/backend";

const USDPL_PORT: number = 54321;

import Carousel from 're-carousel'
import Buttons from "./Buttons";

function TestCarousel({ data }: { data: CardsAndGames }) {
  // return (<div style={{height: "100px", padding: "0px"}}>
  //   <Carousel widgets={[Buttons]}>
  //     <div style={{height: '100%'}}>
  //       Item One
  //       {/* <ButtonItem>Page One</ButtonItem>  */}
  //     </div>
  //     <div style={{height: '100%'}}>
  //       Item Two
  //       {/* <ButtonItem>Page Two</ButtonItem>  */}
  //     </div>
  //     <div style={{height: '100%'}}>
  //       Item Three
  //       {/* <ButtonItem>Page Three</ButtonItem>  */}
  //     </div>
  //   </Carousel>
  // </div>);
  return <div>
    {data.map(([card, games]) => <PanelSectionRow>
      <h1>{card.name}</h1>
      {games.map(game => <p>{game.name}</p>)}
    </PanelSectionRow>)}
  </div>
}

// interface AddMethodArgs {
//   left: number;
//   right: number;
// }

const Content: VFC<{ serverAPI: ServerAPI }> = ({ }) => {
  const { value, refresh } = GetCardsAndGames();

  // const [result, setResult] = useState<number | undefined>();

  // const onClick = async () => {
  //   const result = await serverAPI.callPluginMethod<AddMethodArgs, number>(
  //     "add",
  //     {
  //       left: 2,
  //       right: 2,
  //     }
  //   );
  //   if (result.success) {
  //     setResult(result.result);
  //   }
  // };

  // call hello callback on backend
  // const {value, refresh} = getValue("1234");

  return (
    <div>
      <PanelSection title="DeckyPlugin">
        <PanelSectionRow>
          <h1>Hello, World!</h1>
          <p>
            Response: {
              value ?
                <h1>{value}</h1>
                : "Pending..."
            }
          </p>
          <ButtonItem
            onClick={() => {
              Router.Navigate("/decky-plugin-test");
              Router.CloseSideMenus();
            }}
          >Open Test Page</ButtonItem>
        </PanelSectionRow>
        {value ? <TestCarousel data={value} /> : "Pending..."}
      </PanelSection>
    </div>
  );
};

const DeckyPluginRouterTest: VFC = () => {
  return (
    <div style={{ marginTop: "50px", color: "white" }}>
      Hello World!
      <ButtonItem
        onClick={async () => {
          //@ts-ignore
          window.DeckyPluginLoader.importPlugin("DeckyPlugin", null);
        }}
      >Reload Plugin</ButtonItem>
      <ButtonItem
        onClick={() => {
          Navigation.OpenSideMenu(SideMenu.QuickAccess);
        }}
      >Back</ButtonItem>
    </div>
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  serverApi.routerHook.addRoute("/decky-plugin-test", DeckyPluginRouterTest, {
    exact: true,
  });

  // init USDPL WASM frontend
  // this is required to interface with the backend
  (async () => {
    await init_embedded();
    init_usdpl(USDPL_PORT);
    console.log("USDPL started for framework: " + target_usdpl());
    // Router.Navigate("/decky-plugin-test");
  })();

  return {
    title: <div className={staticClasses.Title}>Example Plugin</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <FaSdCard />,
    onDismount() {
      serverApi.routerHook.removeRoute("/decky-plugin-test");
    },
  };
});
