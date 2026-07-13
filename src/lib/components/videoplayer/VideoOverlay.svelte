<script lang="ts">
    import PauseIcon from 'virtual:icons/material-symbols/pause'
    import PlayIcon from 'virtual:icons/material-symbols/play-arrow'
    import SeekIcon from 'virtual:icons/material-symbols/fast-forward'
    import VolumeMute from 'virtual:icons/ooui/volume-off-ltr'

    const { icon } = $props()
</script>

<div class="absolute top-5 right-5">
    {#if typeof icon === 'number'}
        <p class="font-outline text-5xl text-white">{icon}</p>
    {/if}

    {#if icon === 'playing'}
        <PlayIcon class="text-5xl text-white " />
    {/if}

    {#if icon === 'paused'}
        <PauseIcon class="text-5xl text-white " />
    {/if}

    {#if icon === 'muted'}
        <VolumeMute class="text-5xl text-white" />
    {/if}

    {#if icon === 'forward'}
        <SeekIcon class=" text-5xl text-white" />
    {/if}

    {#if icon === 'rewind'}
        <SeekIcon class="-scale-x-100 transform text-5xl text-white" />
    {/if}
</div>
