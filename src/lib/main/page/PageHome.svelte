<script lang="ts">
    import { writable } from 'svelte/store';

  import PanelMessages from '../panel/messages/PanelMessages.svelte';
  import PanelMessagesSettings from '../panel/messages/PanelMessagesSettings.svelte';
  import type { PanelEntry } from '../panel/panel.js';
  import PanelProvider from '../panel/PanelProvider.svelte';
  import PanelRooms from '../panel/rooms/PanelRooms.svelte';
  import ScreenRoomsSettings from '../panel/rooms/ScreenRoomsSettings.svelte';

    const panels = writable<PanelEntry[]>([
        {
            icon: 'ti ti-bolt',
            name: '接続中',
            componentPanel() {
                return {
                    component: PanelRooms,
                    props: {}
                };
            },
            componentSettings() {
                return { component: ScreenRoomsSettings, props: {} };
            }
        },
        {
            icon: 'ti ti-message',
            name: 'コメント／メッセージ',
            width: 300,
            fit: true,
            componentPanel() {
                return {
                    component: PanelMessages,
                    props: {}
                };
            },
            componentSettings() {
                return {
                    component: PanelMessagesSettings,
                    props: {}
                };
            }
        }
    ]);
</script>

<PanelProvider {panels} />
