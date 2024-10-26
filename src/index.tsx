import {
  ButtonItem,
  definePlugin,
  gamepadDialogClasses,
  joinClassNames,
  PanelSection,
  PanelSectionRow,
  staticClasses,
  ToggleField,
} from "@decky/ui";
import { useEffect, useState } from "react";
import { BiBluetooth, BiUsb } from "react-icons/bi";
import { SiStadia } from "react-icons/si";
import { RiSwitchLine } from "react-icons/ri";
import { FaBatteryEmpty, FaBatteryFull, FaBatteryQuarter, FaBatteryHalf, FaBatteryThreeQuarters, FaPlaystation, FaXbox } from "react-icons/fa";
import { BsController, BsBatteryCharging } from "react-icons/bs";
import { IController } from "./types";
import * as backend from "./backend";
import { IconContext } from "react-icons";
import { setupNotifications } from "./notifications";

const getBatteryIcon = (controller: IController) => {
  if (controller.status === 'charging')
    return <BsBatteryCharging />;

  if (controller.capacity <= 0)
    return <FaBatteryEmpty />;

  else if (controller.capacity <= 25)
    return <FaBatteryQuarter />;

  else if (controller.capacity <= 50)
    return <FaBatteryHalf />;

  else if (controller.capacity <= 75)
    return <FaBatteryThreeQuarters />;

  else
    return <FaBatteryFull />;
};

const getVendorIcon = (controller: IController) => {
  switch (controller.vendorId) {
    case 1356:
      return <FaPlaystation />;

    case 1406:
      return <RiSwitchLine />;

    case 1118:
      return <FaXbox />;

    case 6353: // 0x18D1 = Google
      return <SiStadia />;

    default:
      return <BsController />;
  }
};

const delayPromise = <T,>(value: T) => {
  return new Promise<T>(resolve => {
    setTimeout(() => resolve(value), 275);
  });
};

const Content = () => {
  const [debug, setDebug] = useState<boolean>(false);
  const [notifications, setNotifications] = useState<boolean>(true);
  const [loading, setLoading] = useState<boolean>(false);
  const [controllers, setControllers] = useState<IController[]>([]);
  const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

  // For fetching controller & settings data on render
  useEffect(() => {
    backend.getControllers()
      .then((controllers) => { setControllers(controllers); });

    backend.getDebugSetting()
      .then(debug => { setDebug(debug); });

    backend.getNotificationsSetting()
      .then(notifications => { setNotifications(notifications); });
  }, []);

  const refreshButton = (
    <PanelSectionRow>
      <div className={gamepadDialogClasses.Field}>
        <ButtonItem
          layout="below"
          onClick={async () => {
            setControllers([]);
            setLoading(true);
            setControllers(await delayPromise(backend.getControllers()));
            setLoading(false);
          }}
        >
          Refresh
        </ButtonItem>
      </div>
    </PanelSectionRow >
  );

  const settingsMenu = (
    <PanelSection title="Settings">
      <PanelSectionRow>
        <ToggleField
          label="Notifications"
          checked={notifications}
          onChange={(e: boolean) => {
            backend.setNotificationsSetting(e)
              .then(async () => {
                await backend.settingsCommit();
                setNotifications(e);
              });
          }}
        />
      </PanelSectionRow>
      <PanelSectionRow>
        <ToggleField
          label="Debug mode"
          checked={debug}
          onChange={(e: boolean) => {
            backend.setDebugSetting(e)
              .then(async () => {
                await backend.settingsCommit();
                setDebug(e);
              });
          }}
        />
      </PanelSectionRow>
    </PanelSection>
  )

  if (controllers.length === 0) {
    return <PanelSection title="Controllers">
      <PanelSectionRow>
        <div className={FieldWithSeparator}>
          <div className={gamepadDialogClasses.FieldLabelRow}>
            <div className={gamepadDialogClasses.FieldLabel}>
              {loading ? 'Loading...' : 'No controllers found'}
            </div>
          </div>
        </div>
      </PanelSectionRow>
      {refreshButton}
      {settingsMenu}
    </PanelSection>;
  }

  return (
    <PanelSection title="Controllers">
      {controllers.sort((a, b) => a.name.localeCompare(b.name)).map((controller) => (
        <PanelSectionRow key={controller.productId}>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>
                <IconContext.Provider value={{ style: { verticalAlign: 'middle', marginRight: '10px' } }}>
                  {controller.bluetooth ? <BiBluetooth /> : <BiUsb />}
                </IconContext.Provider>
                <IconContext.Provider value={{ style: { verticalAlign: 'middle', marginRight: '5px' } }}>
                  {getVendorIcon(controller)}
                </IconContext.Provider>
                {controller.name}
              </div>
              {
                (controller.capacity > 0 || controller.status !== "unknown") &&
                <div className={gamepadDialogClasses.FieldChildrenInner}>
                  {
                    // only show battery capacity for non-MS vendors unless capacity is > 0 and over BT
                    // since we don't have the battery capacity yet for Xbox over USB
                    (controller.vendorId != 1118 || (controller.capacity > 0 && controller.bluetooth)) &&
                    <span style={{ display: "inline-block", textAlign: "right", }}>{controller.capacity}%</span>
                  }
                  <IconContext.Provider value={{ style: { verticalAlign: 'middle', marginLeft: "6px" }, size: '2em' }}>
                    {getBatteryIcon(controller)}
                  </IconContext.Provider>
                </div>
              }
            </div>
          </div>
        </PanelSectionRow>
      ))}
      {refreshButton}
      {settingsMenu}
    </PanelSection >
  );
};

export default definePlugin(() => {
  // Starts a self-healing websocket connection to the backend to listen for push notifications.
  // Must be called here to maintain the connection regardless of whether the plugin is open or not
  setupNotifications();

  return {
    title: <div className={staticClasses.Title}>Controller Tools</div>,
    content: <Content />,
    icon: <BsController />,
  };
});
