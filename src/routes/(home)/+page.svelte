<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { onMount } from 'svelte'

    onMount(async () => {
        try {
            const response = await invoke('load_video', {
                url: 'https://dn710604.ca.archive.org/0/items/BigBuckBunny_124/Content/big_buck_bunny_720p_surround.mp4',
            })

            console.log('Video load response:', response)

            if (response.success) {
                await invoke('play_video')
            } else {
                console.error('Failed to load video:', response.error)
            }
        } catch (error) {
            console.error('Error invoking load_video:', error)
        }
    })
</script>

<h1>Home Page</h1>
