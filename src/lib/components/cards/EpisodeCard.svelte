<script lang="ts">
    import { isEpisodeReleased } from '$lib/functions/utility/isEpisodeReleased'
    import InfoIcon from 'virtual:icons/material-symbols/info-outline'
    import EpisodeWatchedButton from '$lib/components/inputs/EpisodeWatchedButton.svelte'

    let {
        episode = $bindable(),
        selectedEpisode = $bindable(),
        currentEpisodeInfo = $bindable(),
        media = $bindable(),
    } = $props()

    const isReleased = $derived(isEpisodeReleased(episode.air_date))
    let imageLoaded = $state(false)

    const isSelected = (): boolean => {
        if (episode?.episode_id === selectedEpisode?.episode_id) return true

        return false
    }

    const handleEpisodeClick = (event: KeyboardEvent | MouseEvent): void => {
        if (selectedEpisode === episode) return
        if (event.type === 'keydown') {
            const keyboardEvent = event as KeyboardEvent
            if (keyboardEvent.key !== 'Enter' && keyboardEvent.key !== ' ') return
            keyboardEvent.preventDefault()
        } else if (event.type !== 'click') return

        selectedEpisode = episode
    }

    const handleInfoClick = (event: KeyboardEvent | MouseEvent): void => {
        event.stopPropagation()

        if (event.type === 'keydown') {
            const keyboardEvent = event as KeyboardEvent
            if (keyboardEvent.key !== 'Enter' && keyboardEvent.key !== ' ') return
            keyboardEvent.preventDefault()
        }

        currentEpisodeInfo = currentEpisodeInfo === episode ? null : episode
    }
</script>

<div
    role="button"
    tabindex="0"
    onkeydown={(event) => handleEpisodeClick(event)}
    onclick={(event) => handleEpisodeClick(event)}
    class="group flex w-full cursor-pointer flex-col px-2 pt-1">
    <div class="relative mb-1 {isSelected() ? 'selected' : ''}">
        {#if currentEpisodeInfo === episode}
            <div class="relative z-10 h-40 w-full overflow-y-auto rounded bg-backgroundColor p-2">
                <p class="text-sm text-textColor">{episode.blurb}</p>
            </div>
        {:else}
            <div
                class="absolute inset-0 z-0 animate-pulse rounded bg-gray-200 dark:bg-gray-800"
                class:hidden={imageLoaded}>
            </div>
            <img
                loading="lazy"
                onload={() => (imageLoaded = true)}
                onerror={() => (imageLoaded = true)}
                class:opacity-0={!imageLoaded}
                class:opacity-100={imageLoaded}
                class="relative z-10 h-40 w-full rounded object-cover"
                src={episode.still_path ? episode.still_path : '/images/offline-placeholder.jpg'}
                alt="Episode Still" />
        {/if}
        {#if currentEpisodeInfo !== episode}
            <div class="absolute top-0 right-0 z-10"><EpisodeWatchedButton bind:episode bind:media /></div>
        {/if}
        {#if isSelected() && imageLoaded}
            <div class="absolute inset-0 -z-10 -m-0.5 rounded bg-linear-to-r from-pink-600 to-primaryColor"></div>
        {/if}
    </div>
    <div class="flex w-full">
        <div class="w-full">
            <p class="flex w-full text-left text-sm {isReleased ? '[&>span]:text-textColor' : '[&>span]:text-red-500'}">
                <span>{episode.episode_num}.&nbsp;</span>
                <span class="flex-1 truncate">{episode.name}</span>
                {#if episode.blurb}
                    <button onclick={(event) => handleInfoClick(event)} onkeydown={(event) => handleInfoClick(event)}
                        ><InfoIcon class="text-md hover:text-primaryColor" /></button>
                {/if}
            </p>
            <p class="w-full pb-2 text-[12px]">
                <span class="text-left {isReleased ? 'text-slate-500' : 'text-red-500'}  uppercase">
                    {episode.air_date}
                </span>
                {#if episode.is_finale}
                    <span class="float-right text-slate-500"> Season Finale </span>
                {/if}
            </p>
        </div>
    </div>
</div>
