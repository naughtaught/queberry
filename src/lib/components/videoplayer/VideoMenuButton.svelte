<script lang="ts">
    import { sessionSettings, videoMetadata } from '$lib/stores/video'
    import MenuIcon from 'virtual:icons/material-symbols/menu'
    import UserRating from '$lib/components/inputs/UserRating.svelte'
    import FavouriteButton from '$lib/components/inputs/FavouriteButton.svelte'
    import HideButton from '$lib/components/inputs/HideButton.svelte'
    import BlacklistSourceButton from '$lib/components/inputs/BlacklistSourceButton.svelte'
    import DownloadButton from '$lib/components/inputs/DownloadButton.svelte'
    import TimingInput from '$lib/components/inputs/TimingInput.svelte'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
    import { settings } from '$lib/stores/user'

    let { currentModal = $bindable() } = $props()

    let media = $derived($videoMetadata.media)

    const episodeTimingsEnabled = $derived(
        $settings.skipIntro || $settings.skipCredits || $settings.skipPreview || $settings.skipRecap,
    )

    const timingsSource = $derived($videoMetadata.media?.type === 'tv' ? $videoMetadata.episode : $videoMetadata.media)
</script>

<button
    aria-label="Video Menu Button"
    class="flex items-center"
    onclick={() => {
        currentModal = currentModal === 'VideoMenu' ? null : 'VideoMenu'
    }}>
    <MenuIcon class="text-2xl text-white transition-colors hover:text-primaryColor" />
</button>

{#if currentModal === 'VideoMenu'}
    <div
        data-tauri-drag-region="false"
        class="fixed z-10 box-border h-auto w-lg cursor-default overflow-x-hidden overflow-y-auto rounded bg-backgroundColor p-4 shadow-lg">
        <div>
            <UserRating bind:media textColor="text-textColor" baseOpacity={60} />
        </div>
        <div class="mt-4 flex justify-between">
            <FavouriteButton bind:media />
            <HideButton bind:media />
            {#if $videoMetadata.resolver?.id}
                <DownloadButton />
            {/if}
            <BlacklistSourceButton infohash={$videoMetadata.infohash} />
        </div>
        {#if episodeTimingsEnabled}
            <div class="mt-3 flex flex-col">
                <Checkbox
                    label="Disable All Timings for Session?"
                    checked={$sessionSettings.disableAllTimings}
                    func={() => {
                        $sessionSettings.disableAllTimings = !$sessionSettings.disableAllTimings
                    }} />
            </div>
            {#if timingsSource && !$sessionSettings.disableAllTimings}
                <div class="mt-4 space-y-3">
                    <TimingInput
                        timings={timingsSource.intro_timings}
                        label="Intro"
                        disableKey="disableIntroTiming"
                        updateKey="isIntroTimingUpdated" />
                    <TimingInput
                        timings={timingsSource.recap_timings}
                        label="Recap"
                        disableKey="disableRecapTiming"
                        updateKey="isRecapTimingUpdated" />
                    <TimingInput
                        timings={timingsSource.preview_timings}
                        label="Preview"
                        disableKey="disablePreviewTiming"
                        updateKey="isPreviewTimingUpdated" />
                    <TimingInput
                        timings={timingsSource.credits_timings}
                        label="Credits"
                        disableKey="disableCreditTiming"
                        updateKey="isCreditTimingUpdated" />
                </div>
            {/if}
        {/if}
    </div>
{/if}
