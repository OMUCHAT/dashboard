<script lang="ts">
    import { ImageContent, Message, RootContent, TextContent } from '@omuchat/client/src/models';
    import { open } from '@tauri-apps/api/dialog';
    import { listen } from '@tauri-apps/api/event';
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import { appWindow } from '@tauri-apps/api/window';
    import { onMount } from 'svelte';

    import { client, type Emoji } from './emoji';
    import EmojiEdit from './EmojiEdit.svelte';
    import EmojiEntry from './EmojiEntry.svelte';

    import InputTextLazy from '$lib/common/input/InputTextLazy.svelte';

    let emojis: Map<string, Emoji> = new Map();
    client.omu.registry.listen<Record<string, Emoji>>(
        { name: 'emojis', app: 'omu.chat.plugins/emoji' },
        (data) => {
            emojis = new Map(Object.entries(data || {}));
        }
    );
    client.run();

    let selectedEmoji: Emoji | undefined;
    let search: string = '';
    let fileDrop: boolean = false;
    let dragFiles: string[] = [];
    let uploading: number = 0;

    async function upload(files: string[]) {
        uploading++;
        await client.omu.endpoints.call({ name: 'upload', app: 'omu.chat.plugins/emoji' }, files);
        uploading--;
    }

    onMount(() => {
        listen('tauri://file-drop', (event) => {
            if (event.windowLabel !== appWindow.label) return;
            fileDrop = false;
            upload(event.payload as string[]);
        });
        listen('tauri://file-drop-hover', (event) => {
            if (event.windowLabel !== appWindow.label) return;
            fileDrop = true;
            dragFiles = event.payload as string[];
        });
        listen('tauri://file-drop-cancelled', (event) => {
            if (event.windowLabel !== appWindow.label) return;
            fileDrop = false;
        });
    });

    function deleteEmoji(event: CustomEvent<Emoji>) {
        const emoji = event.detail;
        emojis.delete(emoji.id);
        client.omu.registry.set(
            { name: 'emojis', app: 'omu.chat.plugins/emoji' },
            Object.fromEntries(emojis)
        );
        if (selectedEmoji?.id === emoji.id) {
            selectedEmoji = undefined;
        }
    }

    function saveEmoji(event: CustomEvent<Emoji>) {
        const emoji = event.detail;
        emojis.set(emoji.id, emoji);
        client.omu.registry.set(
            { name: 'emojis', app: 'omu.chat.plugins/emoji' },
            Object.fromEntries(emojis)
        );
        selectedEmoji = undefined;
    }

    function editEmoji(event: CustomEvent<Emoji>) {
        selectedEmoji = event.detail;
    }

    function testEmoji(event: CustomEvent<Emoji>) {
        const emoji = event.detail;
        client.chat.messages.add(
            new Message({
                id: Date.now().toString(),
                room_id: 'test',
                content: RootContent.of([
                    TextContent.of(`${emoji.name} (${emoji.regex})`),
                    ImageContent.of(emoji.image_url, emoji.id, emoji.name)
                ]),
                created_at: new Date()
            })
        );
    }

    async function openFile() {
        const selected = await open({
            multiple: true,
            filters: [
                {
                    name: 'Images',
                    extensions: ['png', 'webp', 'jpg', 'jpeg']
                }
            ]
        });
        if (!selected) return;
        upload(Array.from(selected));
    }
</script>

<main>
    <div class="header">
        <div class="title">
            <i class="ti ti-icons" />
            絵文字
        </div>
        <div class="search">
            <InputTextLazy placeholder="検索" bind:value={search} />
        </div>
    </div>
    {#if selectedEmoji}
        <div class="emoji-edit">
            <EmojiEdit emoji={selectedEmoji} on:save={saveEmoji} on:delete={deleteEmoji} />
        </div>
    {/if}
    <div class="emojis">
        <div class="upload">
            <small>
                <i class="ti ti-upload" />
                画像をドラッグ&ドロップして追加できます。
            </small>
            <button class="button" on:click={openFile}>
                <i class="ti ti-upload" />
                もしくはファイルを選択
            </button>
            {#if uploading > 0}
                <span>
                    <i class="ti ti-upload" />
                    アップロード中…
                </span>
            {/if}
        </div>
        {#each Array.from(emojis.values()).filter((emoji) => emoji.name.includes(search)) as emoji}
            <EmojiEntry {emoji} on:delete={deleteEmoji} on:edit={editEmoji} on:test={testEmoji} />
        {/each}
    </div>
    <div class="drop" class:active={fileDrop}>
        <div class="tips">
            <i class="ti ti-upload" />
            ドロップして追加
        </div>
        <div class="preview">
            {#each dragFiles as file}
                <img src={convertFileSrc(file)} alt="" />
            {/each}
        </div>
    </div>
</main>

<style lang="scss">
    main {
        display: flex;
        flex-direction: column;
        height: 100%;
        margin: 0;
        background: var(--color-bg-2);
    }

    .header {
        display: flex;
        flex-direction: row;
        gap: 40px;
        align-items: center;
        width: 100%;
        min-height: 80px;
        padding: 20px;
        background: var(--color-bg-2);
        border-bottom: 1px solid var(--color-1);

        .title {
            display: flex;
            flex-direction: row;
            gap: 10px;
            align-items: baseline;
            padding-left: 20px;
            font-size: 18px;
            font-weight: 600;
            color: var(--color-1);
        }
    }

    .emoji-edit {
        display: flex;
        flex-direction: row;
        align-items: center;
        width: 100%;
        padding: 20px;
    }

    .emojis {
        display: flex;
        flex-flow: column;
        gap: 10px;
        width: 100%;
        height: 100%;
        padding: 20px;
        overflow: auto;
        background: var(--color-bg-1);
    }

    .upload {
        display: flex;
        flex-direction: row;
        gap: 10px;
        align-items: center;
        width: 100%;
        height: 40px;

        button {
            height: 32px;
            padding: 0 10px;
            color: var(--color-1);
            cursor: pointer;
            background: var(--color-bg-2);
            border: 1px solid var(--color-1);

            &:hover {
                color: var(--color-bg-2);
                background: var(--color-1);
            }
        }

        span {
            display: flex;
            flex-direction: row;
            gap: 10px;
            align-items: center;
            padding: 0 10px;
            font-size: 12px;
            font-weight: bold;
            color: var(--color-1);
        }
    }

    .drop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
        opacity: 0;

        .tips {
            position: fixed;
            bottom: 20px;
            left: 20px;
            z-index: 1;
            display: flex;
            flex-direction: row;
            gap: 10px;
            align-items: center;
            padding: 10px;
            font-weight: bold;
            color: var(--color-1);
            pointer-events: none;
            background: var(--color-bg-2);
        }

        .preview {
            position: fixed;
            display: table;
            width: 100%;
            height: 100%;
            pointer-events: none;

            img {
                height: 128px;
                padding: 10px;
                border-radius: 5px;
                object-fit: cover;
            }
        }

        &.active {
            pointer-events: all;
            background: color-mix(in srgb, var(--color-bg-1) 90%, transparent 0%);
            opacity: 1;
            transition: all 0.05s ease-in-out;
        }
    }

    small {
        display: flex;
        flex-direction: row;
        gap: 5px;
        align-items: center;
        width: fit-content;
        padding: 5px;
        font-size: 14px;
        font-weight: bold;
        color: var(--color-1);
        background: var(--color-bg-2);

        i {
            font-size: 1.2em;
        }
    }
</style>