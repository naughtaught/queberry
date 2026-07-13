<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { SEEK_AMOUNT } from '$lib/stores/video'
    import SeekIcon from 'virtual:icons/material-symbols/fast-forward'

    const { direction } = $props()

    const label = $derived(direction === 'forward' ? 'Seek Forward' : 'Seek Backwards')
    const svgDirection = $derived(direction === 'forward' ? '' : '-scale-x-100 transform')

    const seek = async (): Promise<void> => {
        const value = direction === 'forward' ? $SEEK_AMOUNT : -Math.abs($SEEK_AMOUNT)

        try {
            const resp = await invokeFunction('seek', {
                value,
            })

            if (!resp.success) throw resp.error
        } catch (error) {
            handleError(error, {
                context: `seeking ${label} failed`,
            })
        }
    }
</script>

<button aria-label={label} onclick={seek}>
    <SeekIcon class="text-3xl {svgDirection} text-white transition-colors hover:text-primaryColor" />
</button>
