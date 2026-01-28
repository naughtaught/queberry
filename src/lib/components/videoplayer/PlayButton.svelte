<script lang="ts">
    import { invokeFunction, videoState } from '$lib'

    const togglePlay = async (): Promise<void> => {
        const response = await invokeFunction('toggle_play', {
            value: $videoState.isPaused,
        })
        if (response.success) $videoState.isPaused = response.data.value
    }
</script>

<button onclick={togglePlay} class="fill-white hover:cursor-pointer">
    {#if $videoState.isPaused}
        <svg height="30px" width="30px" viewBox="0 0 330 330">
            <path
                d="M37.728,328.12c2.266,1.256,4.77,1.88,7.272,1.88c2.763,0,5.522-0.763,7.95-2.28l240-149.999 c4.386-2.741,7.05-7.548,7.05-12.72c0-5.172-2.664-9.979-7.05-12.72L52.95,2.28c-4.625-2.891-10.453-3.043-15.222-0.4 C32.959, 4.524, 30, 9.547, 30, 15v300C30, 320.453, 32.959, 325.476,37.728, 328.12z" />
        </svg>
    {:else}
        <svg width="30px" height="30px" viewBox="0 0 16 16">
            <path d="M7 1H2V15H7V1Z" />
            <path d="M14 1H9V15H14V1Z" />
        </svg>
    {/if}
</button>
