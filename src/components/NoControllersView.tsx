import { gamepadDialogClasses, joinClassNames, PanelSectionRow } from "@decky/ui";

const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

const NoControllersView = () => {
  return (
    <PanelSectionRow>
      <div className={FieldWithSeparator}>
        <div className={gamepadDialogClasses.FieldLabelRow}>
          <div className={gamepadDialogClasses.FieldLabel}>
            No controllers found
          </div>
        </div>
      </div>
    </PanelSectionRow>
  );
};

export default NoControllersView;
