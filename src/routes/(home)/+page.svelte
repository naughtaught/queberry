<script lang="ts">
    import { loadVideo } from '$lib'
</script>

<h1 class="mt-50">Home Page</h1>
<button
    onclick={loadVideo}
    class="m-auto flex items-center justify-center rounded border-2 border-black bg-gray-50 p-1 text-black!"
    >Load Video</button>
