import { writable } from 'svelte/store';

export const scrollPosition = writable<number | undefined>(0)