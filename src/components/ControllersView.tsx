import { gamepadDialogClasses, joinClassNames, PanelSectionRow } from "@decky/ui";
import { IController } from "../types";
import { IconContext } from "react-icons";
import { BiBluetooth, BiUsb } from "react-icons/bi";
import VendorIcon from "./VendorIcon";
import BatteryIcon from "./BatteryIcon";

const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

type ControllersViewProps = {
  controllers: IController[];
};

const ControllersView = ({ controllers }: ControllersViewProps) => {
  return (
    controllers.sort((a, b) => a.name.localeCompare(b.name)).map((controller) => (
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
                  (controller.vendorId != 0x045E || (controller.capacity > 0 && controller.bluetooth)) &&
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
    ))
  );
};

export default ControllersView;
