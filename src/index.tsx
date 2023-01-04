import {
  definePlugin,
  gamepadDialogClasses,
  joinClassNames,
  PanelSection,
  PanelSectionRow,
  ServerAPI,
  staticClasses,
} from "decky-frontend-lib";
import { useEffect, useState, VFC } from "react";
import { BsController } from "react-icons/bs";
import { Controller } from "./types";
import * as backend from "./backend";

const Content: VFC<{ serverAPI: ServerAPI }> = ({ }) => {
  const [controllers, setControllers] = useState<[Controller]>();
  const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

  useEffect(() => {
    const fetchControllers = async () => {
      setControllers(await backend.getControllers());
    };

    fetchControllers();
  }, []);

  return (
    <PanelSection title="Controllers">
      {controllers?.map((controller) => (
        <PanelSectionRow key={controller.productId}>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>
                {controller.name}
              </div>
              <div className={gamepadDialogClasses.FieldChildren}>
                {controller.capacity}% ({controller.status})
              </div>
            </div>
          </div>
        </PanelSectionRow>
      ))}
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
