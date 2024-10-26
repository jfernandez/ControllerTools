import { BsBatteryCharging } from "react-icons/bs";
import { IController } from "../types";
import { FaBatteryEmpty, FaBatteryFull, FaBatteryHalf, FaBatteryQuarter, FaBatteryThreeQuarters } from "react-icons/fa";

type BatteryIconProps = {
  controller: IController;
};

const BatteryIcon = ({ controller }: BatteryIconProps) => {
  return (
    controller.status === "charging" ?
      <BsBatteryCharging /> :
      controller.capacity <= 0 ?
        <FaBatteryEmpty /> :
        controller.capacity <= 25 ?
          <FaBatteryQuarter /> :
          controller.capacity <= 50 ?
            <FaBatteryHalf /> :
              controller.capacity <= 75 ?
                <FaBatteryThreeQuarters />:
                <FaBatteryFull />
  );
};

export default BatteryIcon;
