<script lang="ts">
    import SearchIcon from 'virtual:icons/material-symbols/search'
    import { resolve } from '$app/paths'

    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { modals } from '$lib/stores/app'
    import SearchResultCard from '$lib/components/cards/SearchResultCard.svelte'
    import type { Api } from '$lib/types/api'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { user } from '$lib/stores/user'
    import { onDestroy, onMount } from 'svelte'

    let searchResults: Api.MediaItem[] = $state([])
    let searchText = $state('')
    let hasMore = $state(true)
    let currentPage = $state(0)
    let inputElement: HTMLInputElement
    let hasSearched = $state(false)
    const PAGE_SIZE = 10

    onMount(() => {
        inputElement?.focus()
    })

    const searchMedia = async (): Promise<void> => {
        if (!searchText || !$user) return

        try {
            const response = await invokeFunction('api_search_media', {
                postgresId: $user.postgresId,
                token: $user.token,
                searchTerm: searchText,
                page: currentPage,
                pageSize: PAGE_SIZE,
            })

            if (!response.success) throw response.error

            if (response.data.length < PAGE_SIZE) hasMore = false

            hasSearched = true
            searchResults.push(...response.data)
        } catch (error) {
            hasSearched = false
            handleError(error)
        }
    }

    const resetSearch = (): void => {
        hasMore = true
        currentPage = 0
        searchResults = []
        hasSearched = false
    }

    const handleKeyDown = (e: KeyboardEvent): void => {
        if (e.key === 'Enter') {
            e.preventDefault()
            resetSearch()
            searchMedia()
        }
    }

    const handleSearch = (): void => {
        resetSearch()
        searchMedia()
    }

    onDestroy(() => {
        searchText = ''
        resetSearch()
    })
</script>

<BaseModal position="top" onClose={() => ($modals.search = false)}>
    <div class="min-w-225">
        <div class="border-b border-white/5 p-6">
            <div class="group relative">
                <button
                    onclick={handleSearch}
                    class="absolute top-1/2 right-4 -translate-y-1/2 text-2xl text-textColor">
                    <SearchIcon
                        class="text-xl group-hover:text-primaryColor {$modals.search
                            ? 'text-primaryColor'
                            : 'text-textColor'}" />
                </button>
                <input
                    bind:this={inputElement}
                    bind:value={searchText}
                    onkeydown={handleKeyDown}
                    class="w-full rounded-lg border-0 bg-white/5 py-4 pr-6 pl-4 text-lg focus:ring-2 focus:ring-primaryColor/50"
                    placeholder="Search movies and tv"
                    type="text" />
            </div>
        </div>
        {#if searchResults.length > 0}
            <div class="max-h-[70vh] overflow-y-auto p-2">
                {#each searchResults as result, i (result.id)}
                    <SearchResultCard bind:result={searchResults[i]} />
                {/each}
                <div class="p-6 text-center">
                    {#if hasMore}
                        <button
                            onclick={() => {
                                currentPage++
                                searchMedia()
                            }}
                            class="rounded-full border border-primaryColor/20 px-6 py-2 text-[10px] font-black tracking-[0.3em] text-textColor uppercase transition-colors hover:bg-primaryColor/5 hover:text-primaryColor">
                            Show More
                        </button>
                    {/if}
                </div>
            </div>
        {:else if hasSearched && searchResults.length === 0}
            <div class="max-h-[70vh] p-2">
                <div class="">
                    <a class=" p-6 text-slate-300" onclick={() => ($modals.search = false)} href={resolve('/add', {})}
                        >No results found. Add item?</a>
                </div>
            </div>
        {/if}
    </div>
</BaseModal>
