import { PanelSection } from "@decky/ui";

import { useEffect, useState } from "react";

import NoControllersView from "./NoControllersView";
import RefreshButton from "./RefreshButton";
import SettingsMenu from "./SettingsMenu";

import * as backend from "../backend";
import { IController } from "../types";
import ControllersView from "./ControllersView";

const PluginContent = () => {
  const [debug, setDebug] = useState<boolean>(false);
  const [notifications, setNotifications] = useState<boolean>(true);
  const [controllers, setControllers] = useState<IController[]>([]);

  // For fetching controller & settings data on render
  useEffect(() => {
    backend.getControllers()
      .then(controllers => { setControllers(controllers); });

    backend.getDebugSetting()
      .then(debug => { setDebug(debug); });

    backend.getNotificationsSetting()
      .then(notifications => { setNotifications(notifications); });
  }, []);

  const onRefresh = () => {
    backend
      .getControllers()
      .then(controllers => { setControllers(controllers); });
  };

  const onDebugChange = (e: boolean) => {
    backend.setDebugSetting(e)
      .then(async () => {
        await backend.settingsCommit();
        setDebug(e);
      });
  };

  const onNotificationsChange = (e: boolean) => {
    backend.setNotificationsSetting(e)
      .then(async () => {
        await backend.settingsCommit();
        setNotifications(e);
      });
  };

  return (
    <PanelSection title="Controllers">
      {controllers.length === 0 ?
        <NoControllersView /> :
        <ControllersView controllers={controllers} />}
      <RefreshButton onClick={onRefresh} />
      <SettingsMenu
        debug={debug}
        notifications={notifications}
        onDebugChange={onDebugChange}
        onNotificationsChange={onNotificationsChange}
      />
    </PanelSection>
  );
};

export default PluginContent;
