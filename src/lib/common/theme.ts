import { writable, type Writable } from "svelte/store";

interface Theme {
    [key: string]: string;
}

const theme1: Theme = {
    "--color-bg-1": "#f6f3eb",
    "--color-bg-2": "#fffefc",
    "--color-1": "#0b6f72",
    "--color-2": "#35dfe1",
    "--color-text": "#222",
    "--color-outline": "rgba(0, 0, 0, 0.1)",
    "--margin": "10px",
}

const theme2: Theme = {
    "--color-bg-1": "#f0f0f0",
    "--color-bg-2": "#fffefc",
    // "--color-1": "#404040",
    "--color-1": "#0b6f72",
    "--color-2": "#35dfe1",
    "--color-text": "#222",
    "--color-outline": "rgba(0, 0, 0, 0.1)",
    "--margin": "10px",
}

export const theme: Writable<Theme> = writable(theme2 || theme1);