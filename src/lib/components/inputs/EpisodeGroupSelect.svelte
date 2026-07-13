<script lang="ts">
    import { fetchUpNext } from '$lib/db/fetchUpNext'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { validateUser } from '$lib/functions/user/validateUser'
    import { getFirstUnwatchedEpisode } from '$lib/functions/utility/getFirstUnwatchedEpisode'
    import { getSelectedSeasonsEpisodes } from '$lib/functions/utility/getSelectedSeasonsEpisodes'
    import { selectSeason } from '$lib/functions/utility/selectSeason'
    import { clickOutside } from '$lib/functions/utility/useClickOutside'
    import { onDestroy } from 'svelte'

    let {
        episode_group_keys,
        showWatchedEpisodes,
        media = $bindable(),
        selectedEpisodeGroup = $bindable(),
        selectedSeason = $bindable(),
        selectedSeasonEpisodes = $bindable(),
        selectedEpisode = $bindable(),
    } = $props()

    let isOpen = $state(false)

    const handleMenuClick = (): void => {
        isOpen = !isOpen
    }

    const handleEpisodeGroupClick = async (event: MouseEvent | KeyboardEvent, key: string): Promise<void> => {
        event.stopPropagation()
        try {
            const currentUser = validateUser()
            if (!currentUser) return

            media.episode_group_name = key

            const upsertResponse = await invokeFunction('api_upsert_user_media', {
                postgresId: currentUser.postgresId,
                token: currentUser.token,
                data: {
                    postgresId: currentUser.postgresId,
                    mediaId: media.id,
                    episode_group_name: key,
                },
            })

            if (!upsertResponse.success) throw upsertResponse.error

            selectedSeason = selectSeason(media, showWatchedEpisodes)
            selectedSeasonEpisodes = getSelectedSeasonsEpisodes(selectedSeason, media)
            selectedEpisode =
                getFirstUnwatchedEpisode(selectedSeason, media, showWatchedEpisodes) ?? selectedSeasonEpisodes[0]
            selectedEpisodeGroup = key
            isOpen = false

            await fetchUpNext()
        } catch (error) {
            handleError(error)
        }
    }

    onDestroy(() => {
        isOpen = false
    })
</script>

<button
    use:clickOutside
    onclickOutside={() => {
        isOpen = false
    }}
    onclick={handleMenuClick}
    class="relative w-full border-y border-white/5 py-2 text-center text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">
    {selectedEpisodeGroup ? selectedEpisodeGroup.replaceAll('_', ' ').replace('episodes', '') : 'Episode Groups'}
    {#if isOpen}
        <div
            class="absolute top-full left-0 z-20 mt-0.5 h-fit max-h-[20vh] w-full space-y-3 overflow-y-auto border-b border-white/5 bg-backgroundColor py-6">
            <div
                tabindex="0"
                onclick={(event) => {
                    handleEpisodeGroupClick(event, 'Default')
                }}
                onkeydown={(event) => {
                    handleEpisodeGroupClick(event, 'Default')
                }}
                role="button">
                Default
            </div>
            {#each episode_group_keys as key (key)}
                <div
                    tabindex="0"
                    onclick={(event) => {
                        handleEpisodeGroupClick(event, key)
                    }}
                    onkeydown={(event) => {
                        handleEpisodeGroupClick(event, key)
                    }}
                    class="w-full text-center"
                    role="button">
                    {key.replaceAll('_', ' ').replace('episodes', '')}
                </div>
            {/each}
        </div>
    {/if}
</button>
