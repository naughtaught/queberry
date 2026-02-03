<script lang="ts">
    import { invokeFunction, sessionSettings, Slider, speakerLayoutsWithCenter, videoMetadata, type Api } from '$lib'
    import ArrowRightIcon from 'virtual:icons/material-symbols/arrow-right'
    import ArrowLeftIcon from 'virtual:icons/material-symbols/arrow-left'

    let { bottom, left, currentModal = $bindable() } = $props()

    let isShaderMenuOpen = $state(false)

    const emitFunc = async (emit: string, value: number): Promise<void> => {
        await invokeFunction(emit, { value })
    }

    const toggleShader = async (shader: Api.Shader): Promise<void> => {
        await invokeFunction('toggle_shader', { value: shader.path })
    }
</script>

<div
    class="fixed box-border flex flex-row bg-backgroundColor {bottom} {left} z-10 flex max-h-[80vh] w-auto max-w-125 min-w-48 justify-center overflow-x-hidden overflow-y-auto rounded p-4 shadow-lg">
    <div class="flex flex-col gap-y-3">
        <div class="flex w-full min-w-48 items-center justify-center gap-3">
            <label class="w-full text-center text-xs"
                >Audio Sync Adjust
                <Slider
                    min={-10}
                    max={10}
                    step={0.1}
                    bind:value={$videoMetadata.avSync}
                    func={() => {
                        emitFunc('av_sync_adjust', $videoMetadata.avSync)
                    }}
                    label=""
                    zeroPoint={true} />
            </label>
        </div>
        {#if speakerLayoutsWithCenter.includes($videoMetadata.audioChannel)}
            <div class="flex items-center justify-center gap-3">
                <label class="w-full text-center text-xs"
                    >Center Speaker Level
                    <Slider
                        min={-20}
                        max={20}
                        step={1}
                        bind:value={$sessionSettings.centerSpeakerLevel}
                        func={() => {
                            emitFunc('center_speaker_level', $sessionSettings.centerSpeakerLevel)
                        }}
                        label=""
                        zeroPoint={true} />
                </label>
            </div>
        {/if}
        {#if $videoMetadata.availableShaders.length >= 1}
            <button
                class="float-right mt-5 flex w-full items-center justify-end text-right text-xs text-white transition-colors hover:text-neutral-400"
                onclick={() => {
                    isShaderMenuOpen = !isShaderMenuOpen
                }}
                >Shaders
                <span
                    >{#if !isShaderMenuOpen}<ArrowRightIcon />
                    {:else}<ArrowLeftIcon />{/if}
                </span></button>
        {/if}
    </div>

    {#if isShaderMenuOpen}
        <div class="ml-5 flex flex-col">
            {#each $videoMetadata.availableShaders as shader (shader.filename)}
                <button
                    onclick={() => toggleShader(shader)}
                    type="button"
                    class="flex w-full flex-col items-start gap-y-1 rounded-md px-1 py-1 transition-colors duration-200 hover:bg-gray-800">
                    <div class="flex w-full items-center gap-x-1">
                        <span
                            class="flex h-3 w-3 shrink-0 items-center justify-center rounded-full border-2 {$videoMetadata.activeShaders.some(
                                (x) => x === shader.filename,
                            )
                                ? 'border-primaryColor bg-primaryColor'
                                : 'border-gray-600 bg-transparent'}"
                            aria-label={`Track selection indicator for ${shader.name}`}>
                        </span>
                        <span class="text-sm font-medium text-gray-100">{shader.name}</span>
                    </div>
                </button>
            {/each}
        </div>
    {/if}
</div>
