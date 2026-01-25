<script lang="ts">
    import { type Api } from '$lib'

    let { tracks, currentTrack, bottom, left, currentModal = $bindable(), func } = $props()

    let displayTracks = $derived.by(() => {
        if (currentModal === 'AudioChannels') {
            return tracks.map((track: { value: string; name: string }) => ({
                value: track.value,
                label: track.name || track.value,
                isSelected: track.value === currentTrack,
            }))
        } else if (currentModal === 'Subtitles') {
            return [
                { value: 0, label: 'Off', isSelected: currentTrack === null },
                ...tracks.map((track: Api.SubtitleTrackInfo) => ({
                    value: track.id,
                    label:
                        track.lang && track.title
                            ? `${track.lang} - ${track.title}`
                            : track.lang || track.title || `Track ${track.id}`,
                    isSelected: track.id === currentTrack?.id,
                })),
            ]
        }
        return []
    })
</script>

<div
    class="fixed box-border bg-backgroundColor {bottom} {left} scrollbar scrollbar z-10 max-h-[80vh] w-auto max-w-125 overflow-x-hidden overflow-y-auto rounded p-2 shadow-lg"
    id="track-selection-modal">
    {#if displayTracks.length > 0}
        <div class="flex flex-col gap-1">
            {#each displayTracks as track (track.value)}
                <button
                    type="button"
                    class="flex w-full items-center gap-x-2 rounded-md px-3 py-1 transition-colors duration-200 hover:cursor-pointer hover:bg-gray-800"
                    onclick={() => func(track.value)}
                    aria-label={`Select ${track.label}`}>
                    <span
                        class="flex h-3 w-3 items-center justify-center rounded-full border-2 {track.isSelected
                            ? 'border-primaryColor bg-primaryColor'
                            : 'border-gray-600 bg-transparent '}"
                        tabindex="-1"
                        aria-label={`Track selection indicator for ${track.label}`}></span>
                    <span class="text-sm text-gray-300">{track.label}</span>
                </button>
            {/each}
        </div>
    {/if}
</div>
