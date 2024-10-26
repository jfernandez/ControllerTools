import { FaPlaystation, FaXbox } from "react-icons/fa";
import { IController } from "../types";
import { RiSwitchLine } from "react-icons/ri";
import { SiStadia } from "react-icons/si";
import { BsController } from "react-icons/bs";

type VendorIconProps = {
  controller: IController;
};

const VendorIcon = ({ controller }: VendorIconProps) => {
  return (
    controller.vendorId === 0x054C ? // 0x054C = Sony
      <FaPlaystation /> :
      controller.vendorId === 0x057E ? // 0x057E = Nintendo
        <RiSwitchLine /> :
        controller.vendorId === 0x045E ? // 0x045E = Microsoft
          <FaXbox /> :
          controller.vendorId === 0x18D1 ? // 0x18D1 = Google
            <SiStadia /> :
            <BsController />
  );
};

export default VendorIcon;

