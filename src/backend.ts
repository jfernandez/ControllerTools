import { Controller, Settings } from "./types";

const PORT: number = 33220;
const HOST: string = `http://localhost:${PORT}`;

export async function getControllers(): Promise<[Controller]> {
  let res = await fetch(`${HOST}/controllers`);
  return await res.json();
}

export async function getSettings(): Promise<Settings> {
  let res = await fetch(`${HOST}/settings`);
  return await res.json();
}

export async function setSettings(settings: Settings): Promise<Settings> {
  // Post settings as JSON to server using fetch
  let res = await fetch(`${HOST}/settings`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(settings),
  });

  return await res.json();
}