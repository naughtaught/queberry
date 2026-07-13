import type { Api } from '$lib/types/api'
import { getImagePath } from '$lib/functions/ui/getImagePath'

export const getSeasonPoster = (seasonNum: number | null, seasonContainer: Api.Season[] | null): string | null => {
    if (!seasonNum || !seasonContainer) return null

    const season = seasonContainer.find((s: { season_num: number }) => s.season_num === seasonNum)

    return getImagePath(season?.poster) ?? null
}
