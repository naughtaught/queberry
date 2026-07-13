<script lang="ts">
    import { setVideoVolume } from '$lib/functions/video/setVideoVolume'
    import { settings } from '$lib/stores/user'
    import { sessionSettings } from '$lib/stores/video'
    import VolumeMute from 'virtual:icons/ooui/volume-off-ltr'
    import VolumeMin from 'virtual:icons/teenyicons/volume-1-solid'
    import VolumeMid from 'virtual:icons/teenyicons/volume-2-solid'
    import VolumeMax from 'virtual:icons/teenyicons/volume-3-solid'
    import Slider from '$lib/components/inputs/Slider.svelte'

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
</script>

<button aria-label="Toggle Volume" onclick={toggleMute}>
    {#if $sessionSettings.volume >= 75}
        <VolumeMax class="text-lg text-white transition-colors hover:text-primaryColor" />
    {:else if $sessionSettings.volume >= 50}
        <VolumeMid class="text-lg text-white transition-colors hover:text-primaryColor" />
    {:else if $sessionSettings.volume > 0}
        <VolumeMin class="text-lg text-white transition-colors hover:text-primaryColor" />
    {:else}
        <VolumeMute class="text-lg text-red-400 transition-colors hover:text-primaryColor" />
    {/if}
</button>
<div class="mb-1.5 w-48">
    <Slider
        min={0}
        max={100}
        step={1}
        tooltipSpacing="-1.1rem"
        bind:value={$sessionSettings.volume}
        func={setVolume}
        label="" />
</div>
