<script lang="ts">
    import { CollectionButton, HideButton, UserRating } from '$lib'
    import MenuIcon from 'virtual:icons/material-symbols/menu'

    let isModal = $state(false)
</script>

<button
    aria-label="Video Menu Button"
    class="flex items-center"
    onclick={() => {
        isModal = !isModal
    }}>
    <MenuIcon class="text-2xl text-white transition-colors hover:text-neutral-400" />
</button>

{#if isModal}
    <div
        class="fixed z-10 box-border h-auto w-96 overflow-x-hidden overflow-y-auto rounded bg-backgroundColor p-2 shadow-lg">
        <!-- TODO PASS ITEM & IF RATINGS NOT SET TO HIDDEN-->
        <UserRating textColor="text-white" baseOpacity={60} />
        <div class="mt-4 flex justify-between">
            <!-- TODO PASS ITEM -->
            <CollectionButton label="Collection" textColor="text-white" />
            <!-- TODO PASS ITEM -->
            <HideButton label="Hide" textColor="text-white" />
        </div>
    </div>
{/if}
