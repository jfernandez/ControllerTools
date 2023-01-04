import { Controller } from "./types";

const PORT: number = 33220;
const HOST: string = `http://localhost:${PORT}`;

export async function getControllers(): Promise<[Controller]> {
  let res = await fetch(`${HOST}/controllers`);
  return await res.json();
}