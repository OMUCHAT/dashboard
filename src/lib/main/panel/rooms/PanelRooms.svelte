<script lang="ts">
    import type { models } from '@omuchat/client';
    import { onMount } from 'svelte';

    import RoomEntry from './RoomEntry.svelte';

    import FlexRowWrapper from '$lib/common/FlexRowWrapper.svelte';
    import Button from '$lib/common/input/Button.svelte';
    import { getClient } from '$lib/common/omuchat/client';
    import TableList from '$lib/common/omuchat/TableList.svelte';
    import { screenContext } from '$lib/common/screen/screen';
    import ScreenSetup from '$lib/main/setup/ScreenSetup.svelte';

    export let filter: (key: string, room: models.Room) => boolean = (_, room) => room.online;

    const { chat, client } = getClient();
    let rooms = chat.rooms!.cache;
    let showOffline = false;

    const destroy = chat.rooms.listen((newRooms: Map<string, models.Room>) => {
        rooms = newRooms;
    });
    client.connection.addListener({
        onConnect() {
            chat.rooms.fetch({
                after: 100
            });
        }
    });
    onMount(() => {
        return destroy;
    });

    function toggleOffline() {
        showOffline = !showOffline;
    }

    function openSetup() {
        screenContext.push({
            component: ScreenSetup,
            props: {}
        });
    }
</script>

<div class="rooms">
    {#if rooms.size > 0}
        <TableList table={chat.rooms} component={RoomEntry} {filter} />
        {#if [...rooms.values()].some((room) => !room.online)}
            <Button on:click={toggleOffline}>
                <FlexRowWrapper widthFull reverse>
                    {#if showOffline}
                        <i class="ti ti-chevron-up" />
                        オンラインのみ表示
                    {:else}
                        <i class="ti ti-chevron-down" />
                        オフラインも表示する
                    {/if}
                </FlexRowWrapper>
            </Button>
        {/if}
        {#if showOffline}
            <TableList
                table={chat.rooms}
                component={RoomEntry}
                filter={(_, room) => !room.online}
            />
        {/if}
    {:else}
        <div class="empty">
            ルームが見つかりません！
            <Button on:click={openSetup}>
                チャンネルを追加しますか？
                <i class="ti ti-external-link" />
            </Button>
        </div>
    {/if}
</div>

<style lang="scss">
    .rooms {
        display: flex;
        flex-direction: column;
        height: 100%;
        overflow: auto;
    }

    .empty {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        padding: 40px 0;
    }
</style>
