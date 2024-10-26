import { staticClasses } from "@decky/ui";

type PluginTitleProps = {
  value: string;
};

const PluginTitle = ({ value }: PluginTitleProps) => {
  return (
    <div className={staticClasses.Title}>{value}</div>
  );
};

export default PluginTitle;
