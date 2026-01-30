<script lang="ts">
    import { VideoSettingsModal } from '$lib'
    import SettingsIcon from 'virtual:icons/material-symbols-light/settings'

    let { currentModal = $bindable() } = $props()
</script>

{#if currentModal === 'Settings'}
    <VideoSettingsModal bind:currentModal bottom="bottom-16" left="left-4" />
{/if}

<button
    aria-label="Video Settings"
    onclick={() => {
        currentModal = currentModal === 'Settings' ? null : 'Settings'
    }}>
    <SettingsIcon class="text-white transition-colors hover:text-neutral-400" />
</button>
