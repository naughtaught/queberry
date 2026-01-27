<script lang="ts">
    import { page } from '$app/state'
    import { invalidate } from '$app/navigation'
    import { updateLocalMediaStates, UserRatingButton } from '$lib'

    const totalItems = 10
    const activeIndex = $derived.by(() => {
        return hoveredIndex
        // TODO
        // return hoveredIndex >= 0 ? hoveredIndex : item.user_rating - 1
    })

    let { item = $bindable(), textColor, baseOpacity = 30, onupdate = () => {} } = $props()

    let hoveredIndex = $state(-1)

    const getRatingText = (): string => {
        switch (activeIndex + 1) {
            case 1:
                return 'Abysmal'
            case 2:
                return 'Awful'
            case 3:
                return 'Bad'
            case 4:
                return 'Poor'
            case 5:
                return 'Okay'
            case 6:
                return 'Good'
            case 7:
                return 'Excellent'
            case 8:
                return 'Great'
            case 9:
                return 'Superb'
            case 10:
                return 'Masterpiece'
            default:
                // TODO
                // return item.user_rating ?? ''
                return ''
        }
    }

    const setRating = async (value: number): Promise<void> => {
        const adjustedValue = value + 1

        await updateState(adjustedValue)

        // TODO FIX THIS FOR NEW DB
        // const { error } = await supabase
        //     .from('user_ratings')
        //     .upsert(
        //         { user_id: $user?.supabaseId, media: item.id, rating: adjustedValue },
        //         { onConflict: 'user_id, media' },
        //     )

        // if (error) console.log(error)
    }

    // const clearRating = async (event: Event): Promise<void> => {
    //     event.preventDefault()
    //     await updateState(null)

    //     // TODO FIX THIS FOR NEW DB
    //     // const { error } = await supabase
    //     //     .from('user_ratings')
    //     //     .delete()
    //     //     .eq('user_id', $user?.supabaseId)
    //     //     .eq('media', item.id)

    //     // if (error) console.log(error)
    // }

    const updateState = async (_value: number | null): Promise<void> => {
        // item = { ...item, user_rating: value }

        onupdate(item)

        // TODO
        await updateLocalMediaStates()

        if (page.url.pathname !== '/details') await invalidate('details:data')
    }

    const handleHover = (index: number): void => {
        hoveredIndex = index
    }

    const handleMouseLeave = (): void => {
        hoveredIndex = -1
    }
</script>

<div class="flex w-full items-stretch">
    <div class="flex w-full flex-col justify-between">
        <div class="flex">
            <!-- TODO -->
            <!-- {#if item.user_rating && hoveredIndex < 0}
                <button
                    class="flex h-3 items-center p-0 leading-none"
                    onclick={(event: Event) => {
                        clearRating(event)
                    }}
                    aria-label="Clear rating">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-5 fill-red-600" viewBox="0 -960 960 960"
                        ><path
                            d="M480-80q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q54 0 104-17.5t92-50.5L228-676q-33 42-50.5 92T160-480q0 134 93 227t227 93Zm252-124q33-42 50.5-92T800-480q0-134-93-227t-227-93q-54 0-104 17.5T284-732l448 448Z"
                            stroke="black"
                            stroke-width="25" /></svg>
                </button>
            {:else}
                <div></div>
            {/if} -->
            <p class="{textColor} font-outline min-h-4 text-xs">{getRatingText()}</p>
        </div>
        <div
            class="group flex h-3 w-full items-center rounded"
            onmouseleave={handleMouseLeave}
            role="slider"
            tabindex="0"
            aria-valuemin="0"
            aria-valuemax={totalItems - 1}
            aria-valuenow={activeIndex >= 0 ? activeIndex + 1 : undefined}>
            {#each Array(totalItems) as _, i (i)}
                <UserRatingButton
                    index={i}
                    isActive={activeIndex === i}
                    text={activeIndex >= 0 ? activeIndex + 1 : undefined}
                    {baseOpacity}
                    onHover={handleHover}
                    onClick={setRating} />
            {/each}
        </div>
    </div>
</div>
