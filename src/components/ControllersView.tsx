import { IController } from "../types";
import Controller from "./Controller";

type ControllersViewProps = {
  controllers: IController[];
};

const ControllersView = ({ controllers }: ControllersViewProps) => {
  return (
    controllers
      .sort((a, b) => a.name.localeCompare(b.name))
      .map(controller => (
        <Controller
          controller={controller}
          key={`${controller.vendorId}:${controller.productId}`}
        />
      ))
  );
};

export default ControllersView;
