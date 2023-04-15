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
  TextField,
} from "decky-frontend-lib";
import { Fragment, useRef, VFC } from "react";
import { FaSdCard } from "react-icons/fa";

import logo from "../assets/logo.png";

import { init_usdpl, target_usdpl, init_embedded, call_backend } from "usdpl-front";
import { CardsAndGames, GetCardsAndGames, SetNameForMicroSDCard } from "./hooks/backend";

const USDPL_PORT: number = 54321;

import Carousel from 're-carousel'
import Buttons from "./Buttons";
import Card from "./components/Card";
import PatchPlayButton from "./patch/PatchAppScreen";

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


  return <div style={{ overflow: "scroll" }}>
    {
      data.map(([card, games]) => (
        <Card card={card} games={games}/>
      ))
    }
  </div>
}

// interface AddMethodArgs {
//   left: number;
//   right: number;
// }

const Content: VFC<{ serverAPI: ServerAPI }> = ({ }) => {

  return (
    <div>
      <PanelSection title="DeckyPlugin">
        <PanelSectionRow>
          <h1>Hello, World!</h1>
          <ButtonItem
            onClick={() => {
              Router.Navigate("/decky-plugin-test");
              Router.CloseSideMenus();
            }}
          >Open Test Page</ButtonItem>
        </PanelSectionRow>

      </PanelSection>
    </div>
  );
};

const DeckyPluginRouterTest: VFC = () => {
  const { value, refresh } = GetCardsAndGames();

  return (
    <div style={{ marginTop: "50px", color: "white" }}>
      Hello World!
      <ButtonItem
        onClick={async () => {
          //@ts-ignore
          window.DeckyPluginLoader.importPlugin("MicroSDeck", null);
        }}
      >Reload Plugin</ButtonItem>
      <ButtonItem
        onClick={() => {
          Navigation.OpenSideMenu(SideMenu.QuickAccess);
        }}
      >Back</ButtonItem>

      {value ? <TestCarousel data={value} /> : "Pending..."}
    </div>
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  serverApi.routerHook.addRoute("/decky-plugin-test", DeckyPluginRouterTest, {
    exact: true,
  });

  const patch = PatchPlayButton(serverApi);

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
      serverApi.routerHook.removePatch('/library/app/:appid', patch)
    },
  };
});
