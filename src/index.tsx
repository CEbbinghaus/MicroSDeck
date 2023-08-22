import {
  definePlugin,
  DialogButton,
  PanelSection,
  PanelSectionRow,
  Router,
  ServerAPI,
  staticClasses,
} from "decky-frontend-lib";
import { VFC } from "react";
import { FaSdCard } from "react-icons/fa";

import PatchAppScreen from "./patch/PatchAppScreen";
import MicroSDeckConfigPage from "./pages/Config";

import { CONFIGURATION_PATH } from "./const";
import { Logger } from "./Logging";

const Content: VFC<{ serverAPI: ServerAPI }> = ({ }) => {

  return (
    <div>
      <PanelSection>
        <PanelSectionRow>          
          <DialogButton
            onClick={() => {
              Router.Navigate(CONFIGURATION_PATH);
              Router.CloseSideMenus();
            }}
          >Open Settings Page</DialogButton>
        </PanelSectionRow>
      </PanelSection>
      <PanelSection title="Credits">
        <PanelSectionRow >
            <ul style={{margin: 0, padding: 0, paddingTop: 5, fontSize: 16}}>
              <li>CEbbinghaus</li>
            </ul>
        </PanelSectionRow>
      </PanelSection>
    </div>
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  serverApi.routerHook.addRoute(CONFIGURATION_PATH, MicroSDeckConfigPage, {
    exact: true,
  });

  const patch = PatchAppScreen(serverApi);

  Logger.Log("Started MicroSDeck");

  return {
    title: <div className={staticClasses.Title}>Example Plugin</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <FaSdCard />,
    onDismount() {
      serverApi.routerHook.removeRoute(CONFIGURATION_PATH);
      patch && serverApi.routerHook.removePatch('/library/app/:appid', patch);
    },
  };
});
