<script lang="ts">
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { directories } from '$lib/stores/app'
    import { videoMetadata } from '$lib/stores/video'
    import DownloadIcon from 'virtual:icons/material-symbols/download'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { user } from '$lib/stores/user'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { downloadsInProgress } from '$lib/stores/plugins'
    import { getSeasonPoster } from '$lib/functions/utility/getSeasonPoster'
    import { extractSeasonAndEpisode } from '$lib/functions/utility/extractSeasonAndEpisode'
    import { getImagePath } from '$lib/functions/ui/getImagePath'

    const { size = 'text-2xl', background = '' } = $props()

    let loading = $state(false)

    const downloadFile = async (): Promise<void> => {
        try {
            if (!$videoMetadata.filename || !$videoMetadata?.media || !$user || !$videoMetadata.resolver) return
            loading = true

            const downloadDirectory = $videoMetadata?.media.type === 'tv' ? $directories.tv : $directories.movies
            if (!downloadDirectory) return

            const extracted = extractSeasonAndEpisode($videoMetadata.filename)

            const matchingFileLink = $videoMetadata.files?.find(
                (file) => file.filename === $videoMetadata.filename,
            )?.link

            if (!matchingFileLink || !$videoMetadata.videoUrl)
                throw createError('Could not find matching file link or video URL', 400, { log: false })

            const downloadLink = {
                folderPath: downloadDirectory,
                filename: $videoMetadata.filename,
                fileLink: matchingFileLink,
                imdbId: $videoMetadata.media.imdb_id,
                title: $videoMetadata.media.title,
                released: $videoMetadata.media.released,
                season: extracted?.season ?? null,
                episode: extracted?.episode ?? null,
                fileUrl: $videoMetadata.videoUrl,
                mediaPoster: getImagePath($videoMetadata.media.poster) ?? null,
                seasonPoster: extracted?.season
                    ? getSeasonPoster(extracted?.season, $videoMetadata.media.seasons?.seasons ?? null)
                    : null,
                uuid: crypto.randomUUID(),
                resolverId: $videoMetadata.resolver.id,
                userId: $user.id,
                infoHash: $videoMetadata.infohash
            }

            const response = await invokeFunction('download_file', { params: downloadLink, userId: $user.id })
            if (!response.success) throw response.error

            downloadsInProgress.update((d) => [
                ...d,
                {
                    link: downloadLink,
                    progress: 0,
                    speed: 0,
                    eta: 0,
                    fileIndex: 1,
                    status: 'queued',
                },
            ])
        } catch (error) {
            handleError(error)
        } finally {
            loading = false
        }
    }
</script>

{#if ($videoMetadata.media?.type === 'tv' && $directories.tv) || ($videoMetadata.media?.type === 'movie' && $directories.movies)}
    <button class="flex items-center rounded" aria-label="Download File">
        {#if loading}
            <LoadingSpinner />
        {:else if $downloadsInProgress.find((x) => x.link.filename === $videoMetadata.filename)}
            <DownloadIcon class="{size} rounded text-green-500 {background} hover:cursor-default" />
        {:else}
            <DownloadIcon onclick={downloadFile} class="{size} rounded hover:bg-primaryColor/50 {background} " />
        {/if}
    </button>
{/if}
