import type { App } from '$lib/types/app'

const ONE_HOUR = 60 * 60 * 1000

export const isCacheValid = (routeCache: App.Cache[App.ValidRoutes]): boolean => {
    return routeCache.lastUpdated !== null && Date.now() - routeCache.lastUpdated < ONE_HOUR
}
