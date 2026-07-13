<script lang="ts">
    import { page } from '$app/state'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { indexerSources } from '$lib/stores/plugins'
    import { hashBlacklist, user } from '$lib/stores/user'
    import type { Plugins } from '$lib/types/plugins'
    import BlacklistIcon from 'virtual:icons/healthicons/cancel-24px'

    const { infohash, size = 'text-2xl' } = $props()

    const handleClick = async (event: Event): Promise<void> => {
        event.preventDefault()

        try {
            if (!$user || !infohash) return

            const blacklistResponse = await invokeFunction('create_blacklisted_hash', {
                userId: $user.id,
                hash: infohash,
            })

            if (!blacklistResponse.success) throw blacklistResponse.error

            $indexerSources = [
                ...$indexerSources.map((sourceGroup: Plugins.IndexerSources) =>
                    Object.fromEntries(
                        Object.entries(sourceGroup).map(([id, sourceList]) => [
                            id,
                            sourceList.filter((source) => source.info_hash !== infohash),
                        ]),
                    ),
                ),
            ]

            const blacklistedResponse = await invokeFunction('get_users_blacklisted', {
                userId: $user.id,
            })

            if (blacklistedResponse.success && blacklistedResponse.data.length > 0) {
                $hashBlacklist = blacklistedResponse.data
            }

            if (page.url.pathname === '/video') {
                const resp = await invokeFunction('close_video_player', {})
                if (!resp.success) throw resp.error
            }
        } catch (error) {
            handleError(error)
        }
    }
</script>

<button
    onclick={(event: Event) => {
        handleClick(event)
    }}
    class="flex items-center rounded"
    aria-label="Blacklist Source">
    <BlacklistIcon class="{size} rounded  text-textColor hover:text-red-500" />
</button>
