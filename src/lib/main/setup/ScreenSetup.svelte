<script lang="ts">
    import { ChannelInfo, type ModelJson } from '@omuchat/client';
    import axios from 'axios';

    import ChannelEntry from './ChannelEntry.svelte';

    import Background from '$lib/common/Background.svelte';
    import FlexColWrapper from '$lib/common/FlexColWrapper.svelte';
    import Button from '$lib/common/input/Button.svelte';
    import InputText from '$lib/common/input/InputText.svelte';
    import { getClient } from '$lib/common/omuchat/omuchat';
    import { screenContext } from '$lib/common/screen/screen';
    import Screen from '$lib/common/screen/Screen.svelte';
    import ScreenHeader from '$lib/common/screen/ScreenHeader.svelte';

    const client = getClient();

    interface Response {
        channels: ModelJson<ChannelInfo>[];
    }

    let result: Record<string, { info: ChannelInfo; active: boolean }> | null = null;

    let locked = false;
    let url: string = '';

    function fetchChannels() {
        if (locked) return;
        locked = true;
        axios
            .get<Response>('http://localhost:26423/setup/tree', {
                params: {
                    url
                }
            })
            .then((res) => {
                console.log(res);
                result = res.data.channels.reduce(
                    (acc, channel) => {
                        acc[channel.url] = {
                            info: ChannelInfo.fromJSON(client, channel),
                            active: false
                        };
                        return acc;
                    },
                    {} as Record<string, { info: ChannelInfo; active: boolean }>
                );
            })
            .catch((err) => {
                console.log(err);
            })
            .finally(() => {
                locked = false;
            });
    }

    function finish() {
        if (!result) return;
        const channels = Object.values(result)
            .filter((v) => v.active)
            .map((v) => v.info);
        channels.forEach((channel) => {
            client.channels.add(channel);
        });
        screenContext.pop();
    }

    function reset() {
        result = null;
    }
</script>

<Screen title="setup" windowed={false} noDecorated>
    <Background />
    <div class="container">
        <ScreenHeader title="setup" />
        {#if result}
            <FlexColWrapper>
                <div class="channels">
                    {#each Object.keys(result) as url}
                        <ChannelEntry
                            channel={result[url].info}
                            active={result[url].active}
                            callback={() => {
                                if (!result) return;
                                result[url].active = !result[url].active;
                            }}
                        />
                    {/each}
                </div>
                <div class="buttons">
                    <Button
                        outline
                        rounded
                        callback={finish}
                        disabled={locked || Object.values(result).every((v) => !v.active)}
                    >
                        追加する
                        <i class="ti ti-arrow-right" />
                    </Button>
                    <Button rounded callback={reset} disabled={locked}>
                        <i class="ti ti-arrow-left" />
                        戻る
                    </Button>
                </div>
            </FlexColWrapper>
        {:else}
            <div>
                チャンネルurlを入力するだけ
                <InputText placeholder="url..." bind:value={url} />
            </div>
            <div class="buttons">
                <Button outline rounded callback={fetchChannels} disabled={!url || locked}>
                    次へ
                    <i class="ti ti-arrow-right" />
                </Button>
                <Button rounded callback={screenContext.pop}>
                    スキップ
                    <i class="ti ti-arrow-right" />
                </Button>
            </div>
        {/if}
    </div>
</Screen>

<style lang="scss">
    .container {
        position: fixed;
        top: 40px;
        bottom: 0;
        left: 0;
        display: flex;
        flex-direction: column;
        gap: 60px;
        align-items: center;
        justify-content: flex-start;
        width: 20%;
        min-width: 300px;
        padding-top: 60px;
        padding-right: 40px;
        padding-left: 40px;
        background: var(--color-bg-2);
        outline: 2px solid var(--color-1);
    }

    .channels {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 60%;
        overflow-y: auto;

        &::-webkit-scrollbar {
            display: none;
        }
    }

    .buttons {
        display: flex;
        flex-direction: row-reverse;
        justify-content: space-between;
        width: 100%;
        margin-top: 60px;
    }
</style>