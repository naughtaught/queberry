<script lang="ts">
    import type { Api } from '$lib/types/api.js'
    import { SvelteMap } from 'svelte/reactivity'
    import CarouselCard from '$lib/components/cards/CarouselCard.svelte'
    import PersonFiltersModal from '$lib/components/modals/PersonFiltersModal.svelte'

    const { data } = $props()

    const personData = $derived(data.data)
    let showWatchedMedia = $state(true)
    let showHiddenMedia = $state(true)
    let type = $derived('all types')
    let selectedRole = $derived('all roles')
    const filterText = $derived.by(() => {
        if (selectedRole === 'all roles' && type === 'all types' && showWatchedMedia && showHiddenMedia) {
            return 'All'
        } else {
            return 'Filtered'
        }
    })
    let isFiltersOpen = $state(false)

    const media = $derived.by(() => {
        let filtered = personData.media || []

        const merged = new SvelteMap<number, Api.CastMediaItem>()

        if (!showHiddenMedia) filtered = filtered.filter((x: { hidden: boolean | null }) => !x.hidden)
        if (!showWatchedMedia) filtered = filtered.filter((x: { watched: boolean | null }) => !x.watched)

        filtered.forEach((item: Api.CastMediaItem) => {
            if (merged.has(item.id)) {
                const existing = merged.get(item.id)!
                const existingRoles = Array.isArray(existing.role) ? existing.role : [existing.role]
                const existingChars = Array.isArray(existing.character) ? existing.character : [existing.character]

                existing.role = [...existingRoles, item.role as string]
                existing.character = [...existingChars, item.character as string | null]
            } else {
                const newEntry = {
                    ...item,
                    role: [item.role as string],
                    character: [item.character as string | null],
                }
                merged.set(item.id, newEntry)
            }
        })

        if (selectedRole !== 'all roles') filtered = filtered.filter((x: { watched: boolean | null }) => !x.watched)

        let result = Array.from(merged.values()).sort((a, b) => b.released - a.released)

        if (selectedRole !== 'all roles') {
            result = result.filter((x) => {
                const roles = Array.isArray(x.role) ? x.role : [x.role]
                return roles.some((r: string) => r.toLowerCase() === selectedRole.toLowerCase())
            })
        }
        if (type !== 'all types') result = result.filter((x) => x.type === type)

        return result
    })

    const mediaCount = $derived(media.length)
    const avgRating = $derived(
        media?.length
            ? Math.round(
                  media.reduce(
                      (sum: number, item: { average_rating: number | null }) => sum + (item.average_rating ?? 0),
                      0,
                  ) / media.length,
              )
            : null,
    )
    const ratingLevel = $derived((avgRating ?? 0) === 100 ? 90 : Math.floor((avgRating ?? 0) / 10) * 10)
    const colorVar = $derived(`var(--color-rating-${ratingLevel})`)

    const currentAge = (birthday: string | Date, deathday: string | Date): number => {
        if (deathday) {
            return new Date(deathday).getFullYear() - new Date(birthday).getFullYear()
        } else {
            return new Date().getFullYear() - new Date(birthday).getFullYear()
        }
    }

    const splitIntoParagraphs = (text: string): string[] => {
        return text.split('\n\n')
    }
</script>

{#if isFiltersOpen}
    <PersonFiltersModal bind:isFiltersOpen bind:showWatchedMedia bind:showHiddenMedia bind:selectedRole bind:type />
{/if}

<section class="h-screen w-full overflow-y-auto">
    <section class="relative overflow-hidden px-16 pt-12 pb-24">
        <div class="flex flex-row items-start gap-16">
            <div class="w-112.5 shrink-0">
                <div class="relative">
                    <div
                        class="absolute -inset-1 bg-linear-to-tr from-primaryColor to-transparent opacity-30 blur-2xl transition-opacity">
                    </div>
                    <div class="relative aspect-3/4 overflow-hidden rounded-lg">
                        <img
                            class="h-full w-full object-cover"
                            alt={personData.name}
                            src={personData?.image
                                ? `https://image.tmdb.org/t/p/w300_and_h450_bestv2/${personData?.image}`
                                : '/images/person-placeholder.png'} />
                    </div>
                </div>
            </div>

            <div class="flex-1 space-y-8">
                <div>
                    <h1 class="flex w-full text-8xl leading-none font-black tracking-tighter">
                        {personData.name}
                        {#if avgRating}
                            <span class="relative ml-auto flex h-22 w-22 items-center justify-center">
                                <span class="relative z-10 text-4xl tracking-wide text-black">{avgRating}</span>
                                <svg viewBox="0 0 100 100" class="absolute inset-0 h-full w-full">
                                    <circle cx="50" cy="50" r="48" fill={colorVar} stroke="black" stroke-width="2" />
                                </svg>
                            </span>
                        {/if}
                    </h1>
                    <div class="mt-6 flex flex-wrap gap-8">
                        <div class="flex flex-col">
                            <span class="mb-1 text-xs tracking-[0.3em] text-slate-500 uppercase">Age</span>
                            <span class="text-xl font-medium text-slate-200"
                                >{#if personData?.birthday}{currentAge(
                                        personData?.birthday,
                                        personData?.deathday,
                                    )}{/if}</span>
                        </div>
                        <div class="h-10 w-px self-center bg-white/10"></div>
                        <div class="flex flex-col">
                            <span class="mb-1 text-xs tracking-[0.3em] text-slate-500 uppercase">Born</span>
                            <span class="text-xl font-medium text-slate-200"
                                >{#if personData?.birthday}{new Date(personData?.birthday).toLocaleDateString('en-US', {
                                        month: 'long',
                                        day: 'numeric',
                                        year: 'numeric',
                                    })}
                                {/if}
                            </span>
                        </div>
                        {#if personData?.deathday}
                            <div class="flex flex-col">
                                <span class="mb-1 text-xs tracking-[0.3em] text-slate-500 uppercase">Died</span>
                                <span class="text-xl font-medium text-slate-200"
                                    >{new Date(personData?.deathday).toLocaleDateString('en-US', {
                                        month: 'long',
                                        day: 'numeric',
                                        year: 'numeric',
                                    })}</span>
                            </div>
                        {/if}
                        <div class="h-10 w-px self-center bg-white/10"></div>
                        <div class="flex flex-col">
                            <span class="mb-1 text-xs tracking-[0.3em] text-slate-500 uppercase">Birthplace</span>
                            <span class="text-xl font-medium text-slate-200">{personData?.place_of_birth}</span>
                        </div>
                    </div>
                </div>

                <div class="glass-panel relative overflow-hidden rounded-lg border border-white/5 p-8 shadow-xl">
                    <h3 class="mb-4 text-xs font-bold tracking-[0.3em] text-primaryColor uppercase">Biography</h3>

                    <div class="relative">
                        {#if personData?.biography}
                            <div class="text-lg leading-relaxed font-light text-slate-300 transition-all duration-500">
                                {#each splitIntoParagraphs(personData?.biography) as paragraph, i (i)}
                                    <p class="mb-2">{paragraph}</p>
                                {/each}
                            </div>
                        {:else}
                            <div class="text-lg leading-relaxed font-light text-slate-300">
                                <p>No biography available.</p>
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </section>
    <section class="space-y-12 px-16 pb-32">
        <div class="flex items-end justify-between">
            <div>
                <span class="text-xs font-bold tracking-[0.4em] text-primaryColor uppercase"
                    >{filterText} {mediaCount} items</span>
                <h2 class="font-headline mt-2 text-4xl font-black">Filmography</h2>
            </div>
            <div class="flex gap-4">
                <button
                    onclick={() => {
                        isFiltersOpen = true
                    }}
                    class="rounded-full bg-white/5 px-6 py-2 text-xs font-bold tracking-widest uppercase transition-all hover:bg-white/10"
                    >Filter</button>
                <button
                    onclick={() => {
                        type = 'all types'
                        selectedRole = 'all roles'
                        showHiddenMedia = true
                        showWatchedMedia = true
                    }}
                    class="rounded-full bg-primaryColor/30 px-6 py-2 text-xs font-bold tracking-widest uppercase shadow-lg transition-all hover:bg-primaryColor"
                    >View All</button>
            </div>
        </div>
        <div class="flex flex-wrap gap-8">
            {#if media.length}
                {#each media as item (item.id)}
                    <CarouselCard media={item} updateDetails={false} />
                {/each}
            {:else}
                <p>No matching media found.</p>
            {/if}
        </div>
    </section>
</section>
