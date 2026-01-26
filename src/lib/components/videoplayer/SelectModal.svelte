<script lang="ts">
    import { type Api } from '$lib'

    let { tracks, currentTrack, bottom, left, currentModal = $bindable(), func } = $props()

    const displayTracks = $derived.by(() => {
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
                    metadata: {
                        captionType: track.captionType,
                    },
                    isSelected: track.id === currentTrack?.id,
                })),
            ]
        } else if (currentModal === 'Audio') {
            return [
                { value: 0, label: 'Off', isSelected: currentTrack === null },
                ...tracks.map((track: Api.AudioTrackInfo) => ({
                    value: track.id,
                    label:
                        track.lang && track.title
                            ? `${track.lang} - ${track.title}`
                            : track.lang || track.title || `Track ${track.id}`,
                    metadata: {
                        codec: track.codec,
                        bitrate: track.bitrate,
                        channels: track.channels,
                        sampleRate: track.sampleRate,
                    },
                    isSelected: track.id === currentTrack?.id,
                })),
            ]
        }
        return []
    })

    function formatBitrate(bitrate: number): string {
        if (bitrate >= 1000000) {
            return `${(bitrate / 1000000).toFixed(1)} Mbps`
        } else if (bitrate >= 1000) {
            return `${(bitrate / 1000).toFixed(0)} kbps`
        }
        return `${bitrate} bps`
    }

    function formatSampleRate(sampleRate: number): string {
        if (sampleRate >= 1000) {
            return `${(sampleRate / 1000).toFixed(1)} kHz`
        }
        return `${sampleRate} Hz`
    }
</script>

<div
    class="fixed box-border bg-backgroundColor {bottom} {left} scrollbar scrollbar z-10 max-h-[80vh] w-auto max-w-[75vw] overflow-x-hidden overflow-y-auto rounded p-2 shadow-lg">
    {#if displayTracks.length > 0}
        <div class="flex flex-col gap-1">
            {#each displayTracks as track (track.value)}
                <button
                    type="button"
                    class="flex w-full flex-col items-start gap-y-1 rounded-md px-1 py-1 transition-colors duration-200 hover:cursor-pointer hover:bg-gray-800"
                    onclick={() => func(track.value)}
                    aria-label={`Select ${track.label}`}>
                    <div class="flex w-full items-center gap-x-1">
                        <span
                            class="flex h-3 w-3 shrink-0 items-center justify-center rounded-full border-2 {track.isSelected
                                ? 'border-primaryColor bg-primaryColor'
                                : 'border-gray-600 bg-transparent'}"
                            aria-label={`Track selection indicator for ${track.label}`}>
                        </span>
                        <span class="text-sm font-medium text-gray-100">{track.label}</span>
                    </div>

                    {#if track.metadata}
                        <div class="ml-5 flex flex-wrap gap-x-3 gap-y-1 text-xs text-gray-400">
                            {#if track.metadata.codec}
                                <span class="rounded bg-gray-800 px-1.5 py-0.5 font-mono">
                                    {track.metadata.codec}
                                </span>
                            {/if}
                            {#if track.metadata.bitrate}
                                <span class="rounded bg-gray-800 px-1.5 py-0.5 font-mono">
                                    {formatBitrate(track.metadata.bitrate)}
                                </span>
                            {/if}
                            {#if track.metadata.channels}
                                <span class="rounded bg-gray-800 px-1.5 py-0.5 font-mono">
                                    {track.metadata.channels} ch
                                </span>
                            {/if}
                            {#if track.metadata.sampleRate}
                                <span class="rounded bg-gray-800 px-1.5 py-0.5 font-mono">
                                    {formatSampleRate(track.metadata.sampleRate)}
                                </span>
                            {/if}
                            {#if track.metadata.captionType}
                                <span class="rounded bg-gray-800 px-1.5 py-0.5 font-mono">
                                    {track.metadata.captionType}</span>
                            {/if}
                        </div>
                    {/if}
                </button>
            {/each}
        </div>
    {/if}
</div>
