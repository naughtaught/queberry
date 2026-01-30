<script lang="ts">
    import { sessionSettings, settings, setVideoVolume, Slider } from '$lib'
    import VolumeMute from 'virtual:icons/ooui/volume-off-ltr'
    import VolumeMin from 'virtual:icons/teenyicons/volume-1-solid'
    import VolumeMid from 'virtual:icons/teenyicons/volume-2-solid'
    import VolumeMax from 'virtual:icons/teenyicons/volume-3-solid'

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
        if ($sessionSettings.volume === 0 && previousVolume <= 1) {
            previousVolume = $settings.volume
        }
        setVolume($sessionSettings.volume === 0 ? previousVolume : 0, $sessionSettings.volume, previousVolume)
    }

    // TODO ADD ANOTHER INDICATOR LEVEL @ 25 AND 75
    // TODO ADD VOLUME BOOST?
</script>

<button aria-label="Toggle Volume" onclick={toggleMute}>
    {#if $sessionSettings.volume > 100}
        <VolumeMax class="text-lg text-white transition-colors hover:text-neutral-400" />
    {:else if $sessionSettings.volume > 50}
        <VolumeMid class="text-lg text-white transition-colors hover:text-neutral-400" />
    {:else if $sessionSettings.volume > 0}
        <VolumeMin class="text-lg text-white transition-colors hover:text-neutral-400" />
    {:else}
        <VolumeMute class="text-lg text-red-400 transition-colors hover:text-neutral-400" />
    {/if}
</button>
<div class="mb-1.5 w-48">
    <Slider min={0} max={100} step={1} bind:value={$sessionSettings.volume} func={setVolume} label="" />
</div>
