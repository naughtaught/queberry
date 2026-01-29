<script lang="ts">
    import { invokeFunction, seekAmount, videoState } from '$lib'

    const { direction } = $props()

    const label = $derived(direction === 'forward' ? 'Forward' : 'Back')
    const svgClass = $derived(direction === 'forward' ? '-scale-x-100 transform' : '')

    const seek = async (): Promise<void> => {
        const value = direction === 'forward' ? $seekAmount : -Math.abs($seekAmount)

        await invokeFunction('seek', {
            value,
        })
    }
</script>

<button
    aria-label={label}
    class={$videoState.currentTime >= 1 ? 'fill-white hover:cursor-pointer' : 'fill-neutral-700'}
    onclick={seek}>
    <svg class={svgClass} height="15px" width="15px" viewBox="0 0 330.002 330.002">
        <path
            d="M320.741,1.143c-5.602-2.322-12.057-1.039-16.347,3.251L180.001,128.787V15.001 c0-6.067-3.654-11.537-9.26-13.858c-5.605-2.324-12.059-1.039-16.347,3.251l-150,149.999c-2.813,2.813-4.394,6.628-4.394,10.606 c0,3.978,1.58,7.794,4.394,10.607l150,150.001c2.87,2.87,6.706,4.394,10.61,4.394c1.932,0,3.881-0.374,5.736-1.142 c5.605-2.322,9.26-7.792,9.26-13.858V201.213l124.394,124.394c2.869,2.87,6.706,4.394,10.609,4.394c1.933,0,3.882-0.374,5.737-1.142 c5.605-2.322,9.26-7.792,9.26-13.858v-300C330.001,8.934,326.347,3.465,320.741,1.143z" />
    </svg>
</button>
