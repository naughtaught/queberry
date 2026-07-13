<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { updateCachedMedia } from '$lib/functions/cache/updateCachedMedia'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import CancelIcon from 'virtual:icons/healthicons/cancel-24px'
    import UserRatingButton from '$lib/components/inputs/UserRatingButton.svelte'
    import { validateUser } from '$lib/functions/user/validateUser'

    let { media = $bindable(), textColor, baseOpacity = 30 } = $props()

    const totalItems = 10
    const activeIndex = $derived.by(() => {
        return hoveredIndex >= 0 ? hoveredIndex : media.rating - 1
    })

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
                return media.rating ?? ''
        }
    }

    const setRating = async (value: number): Promise<void> => {
        try {
            const currentUser = validateUser()
            if (!currentUser) return

            const adjustedValue = value + 1

            const upsertResponse = await invokeFunction('api_upsert_user_media', {
                postgresId: currentUser.postgresId,
                token: currentUser.token,
                data: {
                    postgresId: currentUser.postgresId,
                    mediaId: media.id,
                    rating: adjustedValue,
                },
            })

            if (!upsertResponse.success) throw upsertResponse.error

            updateState(adjustedValue)
        } catch (error) {
            handleError(error)
        }
    }

    const clearRating = async (event: Event): Promise<void> => {
        event.preventDefault()
        try {
            const currentUser = validateUser()
            if (!currentUser) return

            const upsertResponse = await invokeFunction('api_upsert_user_media', {
                postgresId: currentUser.postgresId,
                token: currentUser.token,
                data: {
                    postgresId: currentUser.postgresId,
                    mediaId: media.id,
                    rating: null,
                },
            })

            if (!upsertResponse.success) throw upsertResponse.error

            updateState(null)
        } catch (error) {
            handleError(error)
        }
    }

    const updateState = (value: number | null): void => {
        media = { ...media, rating: value }

        updateCachedMedia(media)
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
        <div class="mb-0.5 flex items-center">
            {#if media.rating && hoveredIndex < 0}
                <div class="flex w-full items-center justify-between gap-1 p-0">
                    <span
                        class="{textColor} font-outline text-xs leading-relaxed font-normal tracking-widest text-slate-300">
                        {getRatingText()}
                    </span>
                    <button
                        class="flex items-center gap-1 p-0 leading-none"
                        onclick={(event: Event) => {
                            clearRating(event)
                        }}
                        aria-label="Clear rating">
                        <CancelIcon class="text-xs text-red-500" />
                    </button>
                </div>
            {:else if hoveredIndex >= 0 || media.rating}
                <div class="flex w-full items-center gap-1 p-0">
                    <span
                        class="{textColor} font-outline text-xs leading-relaxed font-normal tracking-widest text-slate-300">
                        {getRatingText()}
                    </span>
                </div>
            {:else}
                <div class="h-5"></div>
            {/if}
        </div>
        <div
            class="group flex h-3 w-full items-center rounded"
            onmouseleave={handleMouseLeave}
            role="slider"
            tabindex="0"
            aria-valuemin="0"
            aria-valuemax={totalItems - 1}
            aria-valuenow={activeIndex >= 0 ? activeIndex + 1 : undefined}>
            {#each Array(totalItems) as _, index (index)}
                <UserRatingButton
                    {index}
                    isActive={activeIndex === index}
                    text={activeIndex >= 0 ? activeIndex + 1 : undefined}
                    {baseOpacity}
                    onHover={handleHover}
                    onClick={setRating} />
            {/each}
        </div>
    </div>
</div>
