<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { handleError, Slider, videoMetadata, type Api } from '$lib'

    let { bottom, left, currentModal = $bindable() } = $props()

    // let shaderMenuOpen = $state(false)

    const adjustAudioVideoSync = async (): Promise<void> => {
        try {
            const response: Api.ApiResponse = await invoke('av_sync_adjust', {
                value: $videoMetadata.avSync,
            })
            if (!response.success) handleError(response.error!)
        } catch (error) {
            const errorDetail: Api.ErrorDetail = {
                code: 500,
                message: error instanceof Error ? error.message : String(error),
                stack: error instanceof Error ? error.stack : undefined,
            }
            handleError(errorDetail)
        }
    }
</script>

<div
    class="fixed box-border bg-backgroundColor {bottom} {left} z-10 flex max-h-[80vh] w-auto max-w-125 flex-col gap-y-3 overflow-x-hidden overflow-y-auto rounded p-4 shadow-lg">
    <!-- {#if !shaderMenuOpen} -->
    <div class="flex w-full items-center justify-center gap-3">
        <label class="text-center text-xs"
            >Audio Sync Adjust
            <Slider
                min={-10}
                max={10}
                step={0.1}
                bind:value={$videoMetadata.avSync}
                func={adjustAudioVideoSync}
                label=""
                zeroPoint={true} />
        </label>
    </div>
    <!-- {#if speakerLayoutsWithCenter.includes($videoState.speakerConfiguration)}
            <div class="flex items-center justify-center gap-3">
                <CenterSpeakerLevelButton change="decrement" />
                <p class="min-w-32.5 text-center text-xs">Center Speaker Volume</p>
                <CenterSpeakerLevelButton change="increment" />
            </div>
        {/if}
        {#if $videoState.currentSubtitleTrack.id && $videoState.currentSubtitleTrack.id > 0}
            <div class="flex items-center justify-center gap-3">
                    <OffsetAdjustButton change={false} label="Decrease Audio Offset" emit="av-sync-adjust" />
        <p class="min-w-32.5 text-center text-xs">Audio Offset</p>
        <OffsetAdjustButton change={true} label="Increase Audio Offset" emit="av-sync-adjust" />

                <OffsetAdjustButton change="decrement" type="Subtitle" />
                <p class="min-w-32.5 text-center text-xs">Subtitle Offset</p>
                <OffsetAdjustButton change="increment" type="Subtitle" />
            </div>
            <div class="flex items-center justify-center gap-3">
                <SubtitleSizeButton change="decrement" />
                <p class="min-w-32.5 text-center text-xs">Subitle Size</p>
                <SubtitleSizeButton change="increment" />
            </div>
        {/if}
        {#if $videoState.availableShaders.length > 1}
            <button
                class="fill-white
        text-xs hover:cursor-pointer"
                onclick={() => {
                    shaderMenuOpen = true
                }}>Select Shader</button>
        {/if} -->
    <!-- {:else} -->
    <!-- <div class="flex flex-col gap-2">
            {#each $videoState.availableShaders as shader}
                <button
                    onclick={() => {
                        EventsEmit('SetShader', shader)
                    }}
                    class="rounded-md bg-gray-800 px-3 py-1 text-sm transition-colors
					duration-200 hover:bg-gray-100
					{shader.isActive ? 'text-primary' : 'text-gray-300 '}">
                    {shader.name}
                </button>
            {/each}
        </div>
        <div class="flex items-center justify-between">
            <button
                class="flex h-6 w-6 items-center justify-center rounded-full transition-colors duration-200 hover:bg-gray-200"
                onclick={() => {
                    shaderMenuOpen = false
                    currentModal === 'Settings'
                }}
                aria-label="Close">
                <span class="text-lg leading-none text-gray-300">×</span>
            </button>
        </div> -->
    <!-- {/if} -->
</div>
