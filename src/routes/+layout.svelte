<script lang="ts">
    import ThemeProvider from '$lib/common/theme/ThemeProvider.svelte';
    import Tooltip from '$lib/common/tooltip/Tooltip.svelte';
    import { i18n } from '$lib/i18n/i18n-context.js';
    import { createI18nUnion } from '$lib/i18n/i18n.js';
    import { DEFAULT_LOCALE, LOCALES } from '$lib/i18n/locales/index.js';
    import { language } from '$lib/main/settings.js';
    import { ClipboardHelper } from '$lib/utils/clipboard-helper.js';
    import './styles.scss';

    async function loadLocale() {
        const lang = await LOCALES[$language].load();
        const fallbackLang = await LOCALES[DEFAULT_LOCALE].load();

        if (lang !== fallbackLang) {
            i18n.set(createI18nUnion([lang, fallbackLang]));
        } else {
            i18n.set(lang);
        }
    }

    language.subscribe(loadLocale);
</script>

<svelte:head>
    <title>Dashboard</title>
    <meta name="description" content="Svelte demo app" />
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="crossorigin" />
    <link
        href="https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@100;200;300;400;500;600;700;800;900&display=swap"
        rel="stylesheet"
    />
    <link rel="stylesheet" href="/icons-webfont/tabler-icons.css" />
    <ThemeProvider />
</svelte:head>

<div class="app">
    <main>
        {#await loadLocale()}
            <div class="loading" data-tauri-drag-region></div>
        {:then}
            <slot />
        {:catch error}
            <div class="error" data-tauri-drag-region>
                <div class="container">
                    <div class="title">
                        <div>
                            <i class="ti ti-alert-small" />
                            エラーが発生しました
                            <small> よければdiscordにてお問い合わせいただければ幸いです。 </small>
                        </div>
                        <small>
                            <b>error occurred</b>
                            please contact us on discord.
                        </small>
                    </div>
                    <button
                        on:click={() => {
                            ClipboardHelper.writeText(
                                JSON.stringify({
                                    message: error.message,
                                    stack: error.stack
                                })
                            );
                        }}
                        class="message"
                    >
                        <Tooltip>
                            <div class="description">
                                クリックしてエラー情報をクリップボードにコピーします。
                            </div>
                        </Tooltip>
                        {error.message}
                        {#if error.stack}
                            <pre>{error.stack}</pre>
                        {/if}
                    </button>
                    <div class="buttons">
                        <button on:click={loadLocale}>
                            <i class="ti ti-reload" />
                            Retry
                        </button>
                        <button
                            on:click={() => {
                                window.close();
                            }}
                        >
                            <i class="ti ti-x" />
                            Close
                        </button>
                    </div>
                </div>
            </div>
        {/await}
    </main>
</div>

<style lang="scss">
    .app {
        display: flex;
        flex-direction: column;
        width: 100vw;
        height: 100vh;
        overflow: hidden;
    }

    main {
        flex: 1;
        overflow: hidden;
    }

    .loading {
        display: flex;
        gap: 10px;
        align-items: center;
        justify-content: center;
        height: 100%;
        font-size: 20px;
        font-weight: bold;
        color: var(--color-1);
        background: var(--color-bg-1);
    }

    .error {
        display: flex;
        flex-direction: column;
        gap: 10px;
        align-items: center;
        justify-content: center;
        height: 100%;
        font-size: 20px;
        color: var(--color-bg-1);
        background: var(--color-1);

        .container {
            display: flex;
            flex-direction: column;
            align-items: flex-start;

            .title {
                display: flex;
                flex-direction: column;
                gap: 0;
                align-items: flex-start;
                padding: 0 10px;
                font-size: 24px;
                line-height: 24px;
                color: var(--color-1);
                background: var(--color-bg-2);

                small {
                    b {
                        margin: 0 5px;
                        margin-left: 30px;
                    }

                    font-size: 14px;
                }
            }

            .message {
                display: flex;
                flex-direction: column;
                align-items: flex-start;
                justify-content: flex-start;
                font-size: 16px;
                color: var(--color-bg-2);
                text-align: start;
                appearance: none;
                background: transparent;
                background: var(--color-1);
                border: none;
                outline: none;

                &:hover {
                    color: var(--color-1);
                    background: var(--color-bg-2);
                }
            }

            .buttons {
                display: flex;
                flex-direction: row;
                gap: 10px;
                align-items: center;
                justify-content: center;
                margin-top: 10px;

                button {
                    display: flex;
                    gap: 10px;
                    align-items: center;
                    justify-content: center;
                    height: 40px;
                    padding: 0 10px;
                    margin-top: 40px;
                    font-size: 16px;
                    color: var(--color-bg-1);
                    cursor: pointer;
                    background: var(--color-1);
                    border: 1px solid var(--color-bg-1);

                    &:hover {
                        color: var(--color-1);
                        background: var(--color-bg-1);
                    }
                }
            }
        }
    }
</style>
