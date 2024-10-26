import { definePlugin } from "@decky/ui";
import { BsController } from "react-icons/bs";

import PluginContent from "./components/PluginContent";
import PluginTitle from "./components/PluginTitle";
import { setupNotifications } from "./notifications";

export default definePlugin(() => {
  // Starts a self-healing websocket connection to the backend to listen for push notifications.
  // Must be called here to maintain the connection regardless of whether the plugin is open or not
  setupNotifications();

  return {
    title: <PluginTitle value="Controller Tools"/>,
    content: <PluginContent />,
    icon: <BsController />,
  };
});
