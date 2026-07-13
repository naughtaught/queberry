<script lang="ts">
    import FullscreenIcon from 'virtual:icons/material-symbols/fullscreen'
    import ExitFullscreenIcon from 'virtual:icons/material-symbols/fullscreen-exit'
    import MinimizeIcon from 'virtual:icons/material-symbols/minimize'
    import CloseIcon from 'virtual:icons/material-symbols/close'
    import { minimizeApp } from '$lib/functions/ui/minimizeApp'
    import { toggleFullscreen } from '$lib/functions/ui/toggleFullscreen'
    import { isAppFullscreen } from '$lib/stores/app'
    import { closeApp } from '$lib/functions/ui/closeApp'
</script>

<div
    class="fixed top-0 z-50 flex h-8 w-screen items-center justify-end bg-transparent pr-3 outline-none"
    data-tauri-drag-region="true"
    role="menubar"
    id="dragbar"
    tabindex="0">
    <div class="group z-50 flex h-full w-fit items-center justify-end rounded px-3 hover:bg-black/50">
        <div
            class="z-50 opacity-0 transition-opacity duration-200 group-focus-within:opacity-100 group-hover:opacity-100">
            <div class="flex gap-x-3">
                <button class="z-50" onclick={minimizeApp} aria-label="Minimize window">
                    <MinimizeIcon class="text-textColor transition-colors hover:text-primaryColor" />
                </button>
                <button class="z-50" onclick={toggleFullscreen} aria-label="Toggle Fullscreen">
                    {#if $isAppFullscreen}
                        <ExitFullscreenIcon class="text-textColor transition-colors hover:text-primaryColor" />
                    {:else}
                        <FullscreenIcon class="text-textColor transition-colors hover:text-primaryColor" />
                    {/if}
                </button>
                <button class="z-50" onclick={closeApp} aria-label="Close App">
                    <CloseIcon class="text-textColor transition-colors hover:text-primaryColor" />
                </button>
            </div>
        </div>
    </div>
</div>
