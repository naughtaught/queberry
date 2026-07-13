import type { Api } from '$lib/types/api'

export const parseCredits = (
    media: Api.MediaItem,
    returnAmount: number,
): { cast: Api.CastMember[]; directors: Api.CastMember[]; writers: Api.CastMember[]; creators: Api.CastMember[] } => {
    const cast =
        media.cast
            ?.filter((person: Api.CastMember) => person.role === 'Cast')
            .sort((a, b) => {
                if (a.billing_order === null && b.billing_order === null) return 0
                if (a.billing_order === null) return 1
                if (b.billing_order === null) return -1
                return a.billing_order - b.billing_order
            })
            .slice(0, returnAmount) ?? []

    const directors =
        media.cast?.filter((person: Api.CastMember) => person.role === 'Director').slice(0, returnAmount) ?? []

    const writers =
        media.cast?.filter((person: Api.CastMember) => person.role === 'Writer').slice(0, returnAmount) ?? []

    const creators =
        media.cast?.filter((person: Api.CastMember) => person.role === 'Creator').slice(0, returnAmount) ?? []

    return {
        cast,
        directors,
        writers,
        creators,
    }
}
