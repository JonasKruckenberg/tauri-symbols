import { writable } from "svelte/store";
import { browser } from "$app/env";
import { invoke } from "@tauri-apps/api/tauri";

export type Icon = [name: string, symbol: string];

export const icons = writable<Icon[]>([]);

if (browser) {
  invoke<Icon[]>("all").then((data) => icons.set(data));
}
