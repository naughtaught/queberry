<script>
    let { tracks, currentTrack, bottom, left, currentModal = $bindable(), func } = $props()
</script>

<div
    class="fixed box-border bg-backgroundColor {bottom} {left} scrollbar scrollbar z-10 max-h-[80vh] w-auto max-w-125 overflow-x-hidden overflow-y-auto rounded p-2 shadow-lg"
    id="track-selection-modal">
    {#if currentModal === 'Speakers'}
        <div class="flex flex-col gap-1">
            {#each tracks as track, index (track.value)}
                <button
                    type="button"
                    class="flex w-full items-center gap-x-2 rounded-md px-3 py-1 transition-colors duration-200 hover:cursor-pointer hover:bg-gray-800"
                    onclick={() => func(track.value)}
                    aria-label={`Select ${track.name || `Track ${index + 1}`}`}>
                    <span
                        class="flex h-3 w-3 items-center justify-center rounded-full border-2 {track.value ===
                        currentTrack
                            ? 'border-primaryColor bg-primaryColor'
                            : 'border-gray-600 bg-transparent '}"
                        tabindex="-1"
                        aria-label={`Track selection indicator for ${track.name || `Track ${index + 1}`}`}></span>
                    <span class="text-sm text-gray-300">{track.name || `Track ${index + 1}`}</span>
                </button>
            {/each}
        </div>
    {/if}
</div>
