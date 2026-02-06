<script lang="ts">
    import { handleError, invokeFunction, seekAmount } from '$lib'
    import SeekIcon from 'virtual:icons/material-symbols/fast-forward'

    const { direction } = $props()

    const label = $derived(direction === 'forward' ? 'Seek Forward' : 'Seek Backwards')
    const svgDirection = $derived(direction === 'forward' ? '' : '-scale-x-100 transform')

    const seek = async (): Promise<void> => {
        const value = direction === 'forward' ? $seekAmount : -Math.abs($seekAmount)

        try {
            const resp = await invokeFunction('seek', {
                value,
            })

            if (resp.error) throw resp.error
        } catch (error) {
            handleError(error, {
                context: `seeking ${label} failed`,
            })
        }
    }
</script>

<button aria-label={label} onclick={seek}>
    <SeekIcon class="text-3xl {svgDirection} text-white transition-colors hover:text-neutral-400" />
</button>
