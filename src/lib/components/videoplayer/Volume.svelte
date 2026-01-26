<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { handleError, settings, Slider, type Api } from '$lib'

    let previousVolume = $state($settings.volume)

    const setVolume = async (): Promise<void> => {
        // TODO emit
        try {
            const response: Api.ApiResponse = await invoke('set_volume', {
                volume: $settings.volume,
            })
            if (response.success) {
                $settings.volume = response.data.volume
            } else {
                handleError(response.error!)
            }
        } catch (error) {
            const errorDetail: Api.ErrorDetail = {
                code: 500,
                message: error instanceof Error ? error.message : String(error),
                stack: error instanceof Error ? error.stack : undefined,
            }
            handleError(errorDetail)
        }
    }

    const toggleMute = (): void => {
        if ($settings.volume > 0) {
            previousVolume = $settings.volume
            $settings.volume = 0
        } else {
            $settings.volume = previousVolume
        }
        setVolume()
    }

    // TODO ADD ANOTHER INDICATOR LEVEL @ 25 AND 75
    // TODO ADD VOLUME BOOST?
</script>

<button
    aria-label="Toggle Volume"
    class="fill-white hover:cursor-pointer"
    onclick={() => {
        toggleMute()
    }}>
    {#if $settings.volume >= 50}
        <svg width="15px" height="15px" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
            <path d="M6 1H8V15H6L2 11H0V5H2L6 1Z" />
            <path
                d="M14 8C14 5.79086 12.2091 4 10 4V2C13.3137 2 16 4.68629 16 8C16 11.3137 13.3137 14 10 14V12C12.2091 12 14 10.2091 14 8Z" />
            <path d="M12 8C12 9.10457 11.1046 10 10 10V6C11.1046 6 12 6.89543 12 8Z" />
        </svg>
    {:else if $settings.volume > 0}
        <svg width="15px" height="15px" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
            <path d="M8 1H6L2 5H0V11H2L6 15H8V1Z" />
            <path d="M12 8C12 9.10457 11.1046 10 10 10V6C11.1046 6 12 6.89543 12 8Z" />
        </svg>
    {:else}
        <svg width="15px" height="15px" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
            <path d="M8 1H6L2 5H0V11H2L6 15H8V1Z" />
            <path
                d="M9.29289 6.20711L11.0858 8L9.29289 9.79289L10.7071 11.2071L12.5 9.41421L14.2929 11.2071L15.7071 9.79289L13.9142 8L15.7071 6.20711L14.2929 4.79289L12.5 6.58579L10.7071 4.79289L9.29289 6.20711Z" />
        </svg>
    {/if}
</button>
<div class="mb-1.5 w-48">
    <Slider min={0} max={100} step={1} bind:value={$settings.volume} func={setVolume} label="" />
</div>
