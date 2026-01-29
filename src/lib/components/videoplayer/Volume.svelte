<script lang="ts">
    import { sessionSettings, setVideoVolume, Slider } from '$lib'

    let { previousVolume = $bindable() } = $props()

    const setVolume = async (
        newVolume: number = $sessionSettings.volume,
        currentVolume: number = previousVolume,
        previous: number = previousVolume,
    ): Promise<void> => {
        const { newValue, previousValue } = await setVideoVolume(newVolume, currentVolume, previous)

        $sessionSettings.volume = newValue
        previousVolume = previousValue
    }

    const toggleMute = (): void => {
        setVolume($sessionSettings.volume === 0 ? previousVolume : 0, $sessionSettings.volume, previousVolume)
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
    {#if $sessionSettings.volume >= 50}
        <svg width="15px" height="15px" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
            <path d="M6 1H8V15H6L2 11H0V5H2L6 1Z" />
            <path
                d="M14 8C14 5.79086 12.2091 4 10 4V2C13.3137 2 16 4.68629 16 8C16 11.3137 13.3137 14 10 14V12C12.2091 12 14 10.2091 14 8Z" />
            <path d="M12 8C12 9.10457 11.1046 10 10 10V6C11.1046 6 12 6.89543 12 8Z" />
        </svg>
    {:else if $sessionSettings.volume > 0}
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
    <Slider min={0} max={100} step={1} bind:value={$sessionSettings.volume} func={setVolume} label="" />
</div>
