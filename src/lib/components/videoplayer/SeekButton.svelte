<script lang="ts">
    import { invokeFunction, seekAmount } from '$lib'
    import SeekIcon from 'virtual:icons/material-symbols/fast-forward'

    const { direction } = $props()

    const label = $derived(direction === 'forward' ? 'Seek Forward' : 'Seek Backwards')
    const svgDirection = $derived(direction === 'forward' ? '' : '-scale-x-100 transform')

    const seek = async (): Promise<void> => {
        const value = direction === 'forward' ? $seekAmount : -Math.abs($seekAmount)

        await invokeFunction('seek', {
            value,
        })
    }
</script>

<button aria-label={label} onclick={seek}>
    <SeekIcon class="text-3xl {svgDirection} text-white transition-colors hover:text-neutral-400" />
</button>
