<script lang="ts">
    import FullscreenIcon from 'virtual:icons/material-symbols/fullscreen'
    import ExitFullscreenIcon from 'virtual:icons/material-symbols/fullscreen-exit'
    import MinimizeIcon from 'virtual:icons/material-symbols/minimize'
    import CloseIcon from 'virtual:icons/material-symbols/close'
    import HomeIcon from 'virtual:icons/ic/baseline-home'
    import { appState, closeApp, HeaderLink, minimizeApp, toggleFullscreen } from '$lib'
    import { resolve } from '$app/paths'
</script>

<header
    class="-z-10 flex w-full max-w-screen items-center justify-between bg-transparent p-2"
    data-tauri-drag-region="true">
    <div class="z-50 flex min-w-34 shrink-0 items-center" data-tauri-drag-region="true">
        <img class="h-12 w-12" src="src/static/logo.svg" alt="" />
    </div>
    <nav class="mx-4 flex w-full flex-1 justify-center" data-tauri-drag-region="true">
        <ul
            data-tauri-drag-region="true"
            class="flex w-full max-w-2xl justify-center gap-x-4 font-semibold lg:gap-x-6 [&>li]:relative [&>li]:z-0 [&>li]:flex">
            <li>
                <a href={resolve('/', {})}>
                    <HeaderLink name="Home">
                        <HomeIcon class="text-xl text-primaryColor" />
                    </HeaderLink>
                </a>
            </li>
        </ul>
    </nav>
    <nav class="flex shrink-0 justify-end">
        <ul class="flex space-x-4 [&>li]:relative [&>li]:flex">
            <li>
                <button onclick={minimizeApp} data-action="minimize">
                    <MinimizeIcon class="text-textColor transition-colors hover:text-neutral-400" />
                </button>
            </li>
            <li class="z-0">
                <button onclick={toggleFullscreen}>
                    {#if $appState.isFullscreen}
                        <ExitFullscreenIcon class="text-textColor transition-colors hover:text-neutral-400" />
                    {:else}
                        <FullscreenIcon class="text-textColor transition-colors hover:text-neutral-400" />
                    {/if}
                </button>
            </li>
            <li>
                <button class="z-50" onclick={closeApp} data-action="close">
                    <CloseIcon class="text-textColor transition-colors hover:text-neutral-400" />
                </button>
            </li>
        </ul>
    </nav>
</header>
