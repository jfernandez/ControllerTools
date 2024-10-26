import {
  ButtonItem,
  gamepadDialogClasses,
  joinClassNames,
  PanelSection,
  PanelSectionRow,
  ToggleField,
} from "@decky/ui";

import { useEffect, useState } from "react";
import { IconContext } from "react-icons";
import { BiBluetooth, BiUsb } from "react-icons/bi";

import * as backend from "../backend";
import { IController } from "../types";
import BatteryIcon from "./BatteryIcon";
import VendorIcon from "./VendorIcon";

const delayPromise = <T,>(value: T) => {
  return new Promise<T>(resolve => {
    setTimeout(() => resolve(value), 275);
  });
};

const PluginContent = () => {
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
                  <VendorIcon controller={controller}/>
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
                    <BatteryIcon controller={controller}/>
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

export default PluginContent;
