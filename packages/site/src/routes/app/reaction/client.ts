import { Client } from '@omuchatjs/chat';
import { setClient } from '@omuchatjs/ui';
import { BROWSER } from 'esm-env';
import getApp from './app.js';

export const client = new Client({
    app: getApp(BROWSER ? window.location.origin : ''),
});
setClient(client);

if (BROWSER) {
    client.start();
}