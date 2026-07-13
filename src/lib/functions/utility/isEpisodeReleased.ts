export const isEpisodeReleased = (air_date: string | null): boolean => {
    if (!air_date) return false
    const today = new Date().toLocaleDateString('en-CA')
    return today >= air_date
}
