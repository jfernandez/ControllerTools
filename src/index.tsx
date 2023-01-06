import {
  ButtonItem,
  definePlugin,
  gamepadDialogClasses,
  joinClassNames,
  PanelSection,
  PanelSectionRow,
  ServerAPI,
  staticClasses,
} from "decky-frontend-lib";
import { useEffect, useState, VFC } from "react";
import { BiBluetooth, BiUsb } from "react-icons/bi";
import { FaBatteryEmpty, FaBatteryFull, FaBatteryQuarter, FaBatteryHalf, FaBatteryThreeQuarters } from "react-icons/fa";
import { BsController, BsBatteryCharging } from "react-icons/bs";
import { Controller } from "./types";
import * as backend from "./backend";
import { IconContext } from "react-icons";

function getBatteryIcon(controller: Controller) {
  if (controller.status === 'charging') {
    return <BsBatteryCharging />;
  }

  if (controller.capacity <= 0) {
    return <FaBatteryEmpty />;
  } else if (controller.capacity <= 25) {
    return <FaBatteryQuarter />;
  } else if (controller.capacity <= 50) {
    return <FaBatteryHalf />;
  } else if (controller.capacity <= 75) {
    return <FaBatteryThreeQuarters />;
  } else {
    return <FaBatteryFull />;
  }
}

async function delayPromise<T>(value: T): Promise<T> {
  return new Promise<T>(resolve => {
    setTimeout(() => resolve(value), 275);
  });
}

const Content: VFC<{ serverAPI: ServerAPI }> = ({ }) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [controllers, setControllers] = useState<Controller[]>([]);
  const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

  useEffect(() => {
    backend.getControllers()
      .then((controllers) => { setControllers(controllers) });
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
    </PanelSection>;
  }

  return (
    <PanelSection title="Controllers">
      {controllers.map((controller) => (
        <PanelSectionRow key={controller.productId}>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>
                <IconContext.Provider value={{ style: { verticalAlign: 'middle', marginRight: '4px' } }}>
                  {controller.bluetooth ? <BiBluetooth /> : <BiUsb />}
                </IconContext.Provider>
                {controller.name}
              </div>
              <div className={gamepadDialogClasses.FieldChildren}>
                <IconContext.Provider value={{ style: { verticalAlign: 'middle', marginRight: '4px' }, size: '2em' }}>
                  {getBatteryIcon(controller)}
                </IconContext.Provider>
                {controller.capacity}%
              </div>
            </div>
          </div>
        </PanelSectionRow>
      ))}
      {refreshButton}
    </PanelSection >
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  return {
    title: <div className={staticClasses.Title}>Controller Tools</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <BsController />,
  };
});
