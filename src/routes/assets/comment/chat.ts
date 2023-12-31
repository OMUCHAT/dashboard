import { Client } from "@omuchat/client";
import { App } from "@omuchat/omu.js";

import { getTabId } from "$lib/utils/browser-helper.js";

const app = new App({
    name: `emoji-${getTabId()}`,
    version: "0.1.0",
    group: "omu.chat.assets",
});
export const client = new Client({
    app,
});
export const chat = client.chat;