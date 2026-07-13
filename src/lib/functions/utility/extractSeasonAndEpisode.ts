export const extractSeasonAndEpisode = (path: string): { season: number; episode: number } | null => {
    if (!path) return null
    const match = path.match(/S(\d+)E(\d+)/i)
    if (match) {
        return {
            season: parseInt(match[1], 10),
            episode: parseInt(match[2], 10),
        }
    }
    return null
}
