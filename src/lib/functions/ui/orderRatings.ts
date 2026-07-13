export const orderRatings = (
    ratings?: { source: string; rating: number; count: number | null }[] | null,
): { source: string; rating: number; count: number | null }[] => {
    const filteredRatings = ratings?.filter((rating) => rating.source !== 'average') ?? []

    const sourceOrder: Record<string, number> = {
        imdb: 0,
        letterboxd: 1,
        metacritic: 2,
        tomatometer: 3,
        popcornmeter: 4,
    }

    return [...filteredRatings].sort((a, b) => {
        return (sourceOrder[a.source] ?? Infinity) - (sourceOrder[b.source] ?? Infinity)
    })
}
