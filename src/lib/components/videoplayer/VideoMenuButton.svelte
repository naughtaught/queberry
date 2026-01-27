<script lang="ts">
    import { CollectionButton, HideButton, UserRating } from '$lib'

    let isModal = $state(false)
</script>

<button
    aria-label="Video Menu Button"
    class="flex items-center hover:cursor-pointer"
    onclick={() => {
        isModal = !isModal
    }}>
    <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#e3e3e3"
        ><path d="M120-240v-80h720v80H120Zm0-200v-80h720v80H120Zm0-200v-80h720v80H120Z" /></svg>
</button>

{#if isModal}
    <div
        class="fixed z-10 box-border h-auto w-96 overflow-x-hidden overflow-y-auto rounded bg-backgroundColor p-2 shadow-lg">
        <!-- TODO PASS ITEM & IF RATINGS NOT SET TO HIDDEN-->
        <UserRating textColor="text-white" baseOpacity={60} />
        <div class="mt-4 flex justify-between">
            <!-- TODO PASS ITEM -->
            <CollectionButton label="Collection" />
            <!-- TODO PASS ITEM -->
            <HideButton label="Hide" />
        </div>
    </div>
{/if}
