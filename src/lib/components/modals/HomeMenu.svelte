<script lang="ts">
    import { clickOutside } from '$lib/functions/utility/useClickOutside'
    import { modals } from '$lib/stores/app'
    import { showHomeHidden } from '$lib/stores/pages'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
</script>

<div
    use:clickOutside={{ ignore: '#home-menu-button' }}
    onclickOutside={() => {
        $modals.homeMenu = false
    }}
    class="fixed top-12 right-10 z-30 flex h-10 w-48 -translate-y-1/2 rounded-md bg-backgroundColor px-2 py-2">
    <div class="mt-1 ml-5">
        <Checkbox label="Include Hidden" bind:checked={$showHomeHidden} func={() => {}} />
    </div>
</div>
