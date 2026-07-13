import { VALID_ROUTES } from '$lib/stores/pages'
import type { App } from '$lib/types/app'

export const isValidRoute = (route: string | undefined): route is App.ValidRoutes => {
    return route !== undefined && VALID_ROUTES.includes(route as App.ValidRoutes)
}
