import { gamepadDialogClasses, joinClassNames, PanelSectionRow } from "@decky/ui";

const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

type NoControllersViewProps = {
  loading: boolean;
};

const NoControllersView = ({ loading }: NoControllersViewProps) => {
  return (
    <PanelSectionRow>
      <div className={FieldWithSeparator}>
        <div className={gamepadDialogClasses.FieldLabelRow}>
          <div className={gamepadDialogClasses.FieldLabel}>
            {loading ? 'Loading...' : 'No controllers found'}
          </div>
        </div>
      </div>
    </PanelSectionRow>
  );
};

export default NoControllersView;
