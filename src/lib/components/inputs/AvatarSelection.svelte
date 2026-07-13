<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { users } from '$lib/stores/user'
    import { convertFileSrc } from '@tauri-apps/api/core'
    import { onMount } from 'svelte'

    let { selectedAvatar = $bindable(), isLoading } = $props()
    let avatars: { name: string; src: string }[] = $state([])

    onMount(async () => {
        try {
            const userAvatars = $users?.map((x) => x.avatar).filter(Boolean) ?? []
            const response = await invokeFunction('get_avatars', {})

            if (!response.success) throw response.error

            const data = response.data as [string, string][]

            avatars = data
                .filter(([name]) => !userAvatars.includes(name))
                .map(([name, path]) => ({ name, src: convertFileSrc(path) }))
                .sort((a, b) => b.name.localeCompare(a.name))
        } catch (error) {
            handleError(error)
        }
    })
</script>

<div class="max-h-120 w-105 px-6 pt-6">
    <div class="mb-6 text-center">
        <p class="mb-1 font-medium text-slate-300">Select your avatar</p>
        <div class="mx-auto h-1 w-24 rounded-full bg-primaryColor"></div>
    </div>
    <div class="grid max-h-[90%] grid-cols-3 gap-3 overflow-y-auto">
        {#each avatars as avatar (avatar.name)}
            <button
                onclick={() => {
                    if (isLoading) return
                    selectedAvatar = avatar.name
                }}
                class="group relative aspect-square cursor-pointer overflow-hidden rounded-xl border-3 hover:border-pink-600 {selectedAvatar ===
                avatar.name
                    ? 'border-primaryColor'
                    : ''}">
                <img alt="User avatar choice" class="h-full w-full object-cover" src={avatar.src} />
            </button>
        {/each}
        <button
            class="flex aspect-square cursor-pointer items-center justify-center rounded-xl border-2 border-dashed border-slate-600 transition-all hover:bg-slate-700">
            <span class="text-slate-500">add</span>
        </button>
    </div>
</div>
