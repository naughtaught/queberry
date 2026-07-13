import { user } from '$lib/stores/user'
import type { Sql } from '$lib/types/sql'
import { get } from 'svelte/store'

export const validateUser = (): Sql.User | null => {
    const currentUser = get(user)

    if (!currentUser) {
        user.set(null)
        return null
    }

    return currentUser
}
