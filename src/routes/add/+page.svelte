<script lang="ts">
    import RequestCard from '$lib/components/cards/RequestCard.svelte'
    import SearchResultCard from '$lib/components/cards/SearchResultCard.svelte'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { user } from '$lib/stores/user'
    import type { Sql } from '$lib/types/sql'
    import type { Api } from '$lib/types/api'
    import { onDestroy, onMount } from 'svelte'
    import SearchIcon from 'virtual:icons/material-symbols/search'

    const PAGE_SIZE = 1
    const IMDB_REGEX = /^tt\d{7,8}$/i
    const ERROR_TIMEOUT = 5000
    const POLLING_TIME = 10000

    let imdbId = $state<string | null>(null)
    let tmdbId = $state<number | null>(null)
    let tvdbId = $state<number | null>(null)
    let requests: Sql.MediaRequest[] = $state([])
    let searchResult: Api.MediaItem | null = $state(null)
    let hasSearched = $state(false)
    let tmdbType: 'movie' | 'tv' = $state('movie')
    let tvdbType: 'movie' | 'tv' = $state('movie')
    let errorMessage: string | null = $state(null)
    let pollingInterval: ReturnType<typeof setInterval> | null = $state(null)
    let isPolling = $state(false)
    const hasActiveRequests = $derived(
        requests.some((x) => x.status.includes('pending') || x.status.includes('processing')),
    )

    $effect(() => {
        if (hasActiveRequests && !isPolling) {
            startPolling()
        } else if (!hasActiveRequests && isPolling) {
            stopPolling()
        }

        return () => {
            if (!hasActiveRequests) {
                stopPolling()
            }
        }
    })

    const startPolling = (): void => {
        if (isPolling) return

        stopPolling()
        isPolling = true

        pollingInterval = setInterval(() => {
            fetchRequests()
        }, POLLING_TIME)
    }

    const stopPolling = (): void => {
        if (pollingInterval) {
            clearInterval(pollingInterval)
            pollingInterval = null
        }
        isPolling = false
    }

    const fetchRequests = async (): Promise<void> => {
        if (!$user) return

        try {
            const response = await invokeFunction('api_get_media_requests', {
                postgresId: $user.postgresId,
                token: $user.token,
                status: null,
            })
            if (!response.success) throw response.error
            requests = Array.isArray(response.data) ? response.data : []
        } catch (error) {
            handleError(error)
        }
    }

    $effect(() => {
        if (imdbId) errorMessage = null
    })

    $effect(() => {
        if (errorMessage) {
            const timeout = setTimeout(() => {
                errorMessage = null
            }, ERROR_TIMEOUT)

            return () => clearTimeout(timeout)
        }
    })

    const reset = (): void => {
        imdbId = null
        tmdbId = null
        tvdbId = null
        tmdbType = 'movie'
        tvdbType = 'movie'
        hasSearched = false
    }

    const toggleMediaType = (type: 'tmdbType' | 'tvdbType'): void => {
        if (type === 'tmdbType') {
            tmdbType = tmdbType === 'movie' ? 'tv' : 'movie'
        } else {
            tvdbType = tvdbType === 'movie' ? 'tv' : 'movie'
        }
    }

    const searchByImdbId = async (id: string): Promise<Api.MediaItem | null> => {
        if (!$user) return null

        const response = await invokeFunction('api_search_media', {
            postgresId: $user.postgresId,
            token: $user.token,
            searchTerm: id,
            page: 0,
            pageSize: PAGE_SIZE,
        })

        if (!response.success) throw response.error
        return response.data?.length === 1 ? response.data[0] : null
    }

    const checkBlacklist = async (id: string): Promise<Api.Blacklist | null> => {
        if (!$user) return null

        const response = await invokeFunction('api_get_blacklist_entry', {
            postgresId: $user.postgresId,
            token: $user.token,
            imdbId: id,
        })

        if (!response.success) throw response.error
        return response.data ?? null
    }

    const handleBlacklistReason = async (reason: string | null): Promise<void> => {
        if (reason === '-') {
            errorMessage = 'Media not eligible'
            reset()
            return
        }

        if (reason && IMDB_REGEX.test(reason)) {
            const result = await searchByImdbId(reason)
            if (result) searchResult = result
            return
        }

        hasSearched = true
    }

    const fetchImdbId = async (): Promise<void> => {
        if (!imdbId || !$user) return
        if (!IMDB_REGEX.test(imdbId)) {
            errorMessage = 'Not a valid IMDb ID'
            imdbId = null
            return
        }

        searchResult = null
        tmdbId = null
        tvdbId = null
        errorMessage = null

        try {
            const mediaItem = await searchByImdbId(imdbId)

            if (mediaItem) {
                searchResult = mediaItem
                imdbId = null
                return
            }

            const blacklistEntry = await checkBlacklist(imdbId)

            if (blacklistEntry) {
                await handleBlacklistReason(blacklistEntry.reason)
            } else {
                hasSearched = true
            }
        } catch (error) {
            handleError(error)
        }
    }

    const addRequest = async (): Promise<void> => {
        if ((!tmdbId && !tvdbId) || !$user) return

        try {
            const response = await invokeFunction('api_request_media', {
                postgresId: $user.postgresId,
                token: $user.token,
                imdbId,
                tmdbId,
                tmdbType,
                tvdbId,
                tvdbType,
            })

            if (!response.success) throw response.error
            requests = [response.data, ...requests]
        } catch (error) {
            handleError(error)
        } finally {
            reset()
            setTimeout(() => fetchRequests(), 500)
        }
    }

    const cancelRequest = async (requestId: number): Promise<void> => {
        if (!$user) return

        try {
            const response = await invokeFunction('api_delete_media_request', {
                postgresId: $user.postgresId,
                token: $user.token,
                requestId,
            })

            if (!response.success) throw response.error
            requests = requests.filter((x) => x.id !== requestId)
        } catch (error) {
            handleError(error)
        }
    }

    onMount(() => {
        fetchRequests()
    })

    onDestroy(() => {
        stopPolling()
    })
</script>

<section class="relative h-screen w-full max-w-full flex-col space-y-10 overflow-y-auto p-16">
    <header>
        <h2 class="text-5xl font-black tracking-tighter">Add Media</h2>
    </header>
    <div class="flex">
        <div>
            <label
                class="group relative flex w-96 flex-col gap-1.5 text-xs font-semibold tracking-wider text-slate-400 uppercase">
                IMDb ID
                <div class="group relative">
                    <div>
                        <button
                            onclick={fetchImdbId}
                            class="absolute top-1/2 right-4 -translate-y-1/2 text-2xl text-textColor">
                            <SearchIcon
                                class="text-xl {hasSearched
                                    ? 'cursor-default! text-slate-500'
                                    : 'hover:text-primaryColor'} " />
                        </button>
                        <input
                            required
                            disabled={hasSearched}
                            class="{hasSearched
                                ? 'text-slate-500'
                                : 'text-slate-100'} peer w-full rounded-lg border border-slate-700 bg-slate-800/50 px-4 py-3 not-only:placeholder:text-slate-500"
                            placeholder="tt5805752"
                            bind:value={imdbId}
                            type="text" />
                    </div>
                </div>
            </label>
            {#if errorMessage}
                <p class="text-red-500">{errorMessage}</p>
            {:else}
                <div class="h-6"></div>
            {/if}
        </div>

        {#if searchResult}
            <div class="w-full text-xs font-semibold tracking-wider text-slate-400 uppercase">
                <button onclick={() => (searchResult = null)} class="float-right my-auto h-full items-center"
                    >Reset</button>
            </div>
        {/if}
    </div>
    {#if hasSearched && !searchResult}
        <section class="flex items-end space-x-8">
            <label
                class="group relative flex w-96 flex-col gap-1.5 text-xs font-semibold tracking-wider text-slate-400 uppercase">
                TMDB ID
                <div class="group relative flex">
                    <button
                        onclick={() => toggleMediaType('tmdbType')}
                        class="w-20 cursor-pointer rounded-l-lg border border-r-0 border-slate-700 bg-slate-800/50 px-2 py-3 text-sm font-normal tracking-normal text-slate-100 focus:outline-none"
                        >{tmdbType.toUpperCase()}</button>
                    <input
                        required
                        class="peer w-full rounded-r-lg border border-slate-700 bg-slate-800/50 px-4 py-3 text-slate-100 placeholder:text-slate-500"
                        placeholder="403431"
                        bind:value={tmdbId}
                        min="1"
                        type="number" />
                </div>
            </label>
            <label
                class="group relative flex w-96 flex-col gap-1.5 text-xs font-semibold tracking-wider text-slate-400 uppercase">
                TVDB ID
                <div class="group relative flex">
                    <button
                        onclick={() => toggleMediaType('tvdbType')}
                        class="w-20 cursor-pointer rounded-l-lg border border-r-0 border-slate-700 bg-slate-800/50 px-2 py-3 text-sm font-normal tracking-normal text-slate-100 focus:outline-none"
                        >{tvdbType.toUpperCase()}</button>
                    <input
                        required
                        class="peer w-full rounded-r-lg border border-slate-700 bg-slate-800/50 px-4 py-3 text-slate-100 placeholder:text-slate-500"
                        placeholder="6994"
                        bind:value={tvdbId}
                        min="1"
                        type="number" />
                </div>
            </label>
            <div class="flex space-x-8 pb-px">
                <button
                    disabled={!tmdbId && !tvdbId}
                    onclick={addRequest}
                    class="{!tmdbId && !tvdbId
                        ? 'cursor-default! text-neutral-700'
                        : 'text-white hover:text-primaryColor'} flex-1 rounded-lg bg-slate-800 px-6 py-3 font-bold shadow-lg transition-all"
                    >Add</button>
                <button class="text-slate-400 hover:text-slate-200" onclick={reset}>Cancel</button>
            </div>
        </section>
    {:else if searchResult}
        <div class="w-full"><SearchResultCard bind:result={searchResult} /></div>
    {/if}
    {#if !searchResult && !hasSearched}
        <div
            class="glass-panel col-span-12 overflow-hidden rounded-lg border border-white/5 shadow-2xl shadow-black/50">
            <div class="grid grid-cols-12 bg-white/5 px-8 py-4">
                <div class="col-span-4">
                    <span class="text-[10px] font-bold tracking-[0.3em] text-slate-500 uppercase">Media</span>
                </div>
                <div class="col-span-2">
                    <span class="text-[10px] font-bold tracking-[0.3em] text-slate-500 uppercase">Status</span>
                </div>
                <div class="col-span-1 text-center">
                    <span class="text-[10px] font-bold tracking-[0.3em] text-slate-500 uppercase">TMDB ID</span>
                </div>
                <div class="col-span-1 text-center">
                    <span class="text-[10px] font-bold tracking-[0.3em] text-slate-500 uppercase">TVDB ID</span>
                </div>
                <div class="col-span-2 text-right">
                    <span class="text-[10px] font-bold tracking-[0.3em] text-slate-500 uppercase">Requested</span>
                </div>
                <div class="col-span-2 text-right">
                    <span class="text-[10px] font-bold tracking-[0.3em] text-slate-500 uppercase">Action</span>
                </div>
            </div>
            <div>
                {#each requests as request (request.id)}
                    <RequestCard {request} {cancelRequest} />
                {/each}
            </div>
        </div>
    {/if}
</section>
