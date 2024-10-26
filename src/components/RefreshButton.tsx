import {
  ButtonItem,
  PanelSectionRow,
  gamepadDialogClasses,
} from "@decky/ui";

type RefreshButtonProps = {
  onClick: (e: MouseEvent) => void;
};

const RefreshButton = ({ onClick }: RefreshButtonProps) => {
  return (
    <PanelSectionRow>
      <div className={gamepadDialogClasses.Field}>
        <ButtonItem
          layout="below"
          onClick={onClick}
        >
          Refresh
        </ButtonItem>
      </div>
    </PanelSectionRow >
  );
};

export default RefreshButton;
