import { PanelSection, PanelSectionRow, ToggleField } from "@decky/ui";

type SettingsMenuProps = {
  debug: boolean;
  notifications: boolean;
  onDebugChange: (value: boolean) => void;
  onNotificationsChange: (value: boolean) => void;
};

const SettingsMenu = ({ debug, notifications, onDebugChange, onNotificationsChange }: SettingsMenuProps) => {
  return (
    <PanelSection title="Settings">
      <PanelSectionRow>
        <ToggleField
          label="Notifications"
          checked={notifications}
          onChange={onNotificationsChange}
        />
      </PanelSectionRow>
      <PanelSectionRow>
        <ToggleField
          label="Debug mode"
          checked={debug}
          onChange={onDebugChange}
        />
      </PanelSectionRow>
    </PanelSection>
  );
};

export default SettingsMenu;
