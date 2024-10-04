import { callable } from "@decky/api";
import { Controller } from "./types";

const PORT: number = 33220;
const HOST: string = `http://localhost:${PORT}`;

export const getDebugSetting = async () => await callable<[string, boolean], boolean>("settings_getSetting")("debug", false);
export const getNotificationsSetting = async () => await callable<[string, boolean], boolean>("settings_getSetting")("notifications", true);
export const setDebugSetting = async (value: boolean) => await callable<[string, boolean], unknown>("settings_setSetting")("debug", value);
export const setNotificationsSetting = async (value: boolean) => await callable<[string, boolean], unknown>("settings_setSetting")("notifications", value);
export const settingsCommit = callable<[], unknown>("settings_commit");

export async function getControllers(): Promise<[Controller]> {
  let res = await fetch(`${HOST}/controllers`);
  return await res.json();
}
