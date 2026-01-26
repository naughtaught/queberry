<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { videoMetadata, SelectModal, handleError, type Api } from '$lib'

    let { currentModal = $bindable() } = $props()

    const setAudioTrack = async (trackId: number): Promise<void> => {
        try {
            const response: Api.ApiResponse = await invoke('set_audio_track', {
                audioTrackId: trackId,
            })
            if (response.success) {
                $videoMetadata.currentAudioTrack = response.data!.current_audio_track
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
</script>

{#if currentModal === 'Audio'}
    <SelectModal
        bind:currentModal
        tracks={$videoMetadata.audioTracks}
        currentTrack={$videoMetadata.currentAudioTrack}
        func={(trackId: number) => {
            setAudioTrack(trackId)
        }}
        bottom="bottom-16"
        left="left-12" />
{/if}

<button
    aria-label="Audio Track"
    class={$videoMetadata.audioTracks?.length > 1 ? 'fill-white hover:cursor-pointer' : 'fill-neutral-700'}
    onclick={() => {
        if ($videoMetadata.audioTracks?.length > 1) {
            if (currentModal === 'Audio') {
                currentModal = null
            } else {
                currentModal = 'Audio'
            }
        }
    }}>
    <svg width="18px" height="18px" viewBox="0 0 512 512">
        <path
            d="M426.666667,85.3333333 L426.666667,341.333333 L362.626302,341.333333 L362.666667,405.333333 L256,341.333333 L170.666667,341.333333 L170.666667,85.3333333 L426.666667,85.3333333 Z M256,1.42108547e-14 L256,64 L213.333,64 L213.333333,42.6666667 L42.6666667,42.6666667 L42.6666667,213.333333 L149.333,213.333 L149.333,268.8 L64,320 L64.0403648,256 L6.39488462e-14,256 L6.39488462e-14,1.42108547e-14 L256,1.42108547e-14 Z M384,128 L213.333333,128 L213.333333,298.666667 L384,298.666667 L384,128 Z M311.198683,149.333333 L359.616467,277.333333 L335.768901,277.333333 L322.580475,240.658669 L274.524018,240.658669 L261.425923,277.333333 L238.933333,277.333333 L286.267137,149.333333 L311.198683,149.333333 Z M298.552247,170.741943 C296.817878,176.812232 294.528512,183.826018 291.684148,191.7833 L291.325712,192.782875 L280.576241,223.134321 L316.43792,223.134321 L305.68845,192.782875 L304.747024,190.067278 C302.566831,183.717713 300.501905,177.275935 298.552247,170.741943 Z M138.364283,55.8724491 L138.363691,66.5384491 L149.332691,66.5384491 L149.334032,145.217282 C147.846623,148.082062 146.253419,150.895209 144.554383,153.656286 C146.072758,154.996689 147.66531,156.330498 149.332056,157.657476 L149.332744,183.9067 C142.782625,179.623374 136.879514,175.218148 131.623873,170.685181 C117.063661,186.063317 97.230366,196.963418 72.3795207,203.419113 L66.0115407,204.951778 L61.383691,184.126454 C85.6428706,178.735525 103.970928,169.143885 116.711981,155.39526 C105.111587,141.185042 96.9168733,125.119906 92.1670974,107.291622 L90.6021236,100.779065 L111.459775,96.2991661 C114.703867,111.403107 120.706878,124.963276 129.507523,137.067333 C137.440289,122.406679 142.049701,106.041819 143.329049,87.8734181 L63.6976158,87.8724491 L63.6976158,66.5391157 L117.030691,66.5384491 L117.030949,55.8724491 L138.364283,55.8724491 Z">
        </path>
    </svg>
</button>
