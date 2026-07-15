<script lang="ts">
    import Checkbox from '../inputs/Checkbox.svelte'
    import Select from '../inputs/Select.svelte'
    import BaseModal from './BaseModal.svelte'
    import ResetIcon from 'virtual:icons/ri/reset-left-fill'

    const types = [
        { name: 'All Types', value: 'all types' },
        { name: 'Movies', value: 'movie' },
        { name: 'TV', value: 'tv' },
    ]

    const roles = [
        { name: 'All Roles', value: 'all roles' },
        { name: 'Cast', value: 'cast' },
        { name: 'Creator', value: 'creator' },
        { name: 'Director', value: 'director' },
        { name: 'Writer', value: 'writer' },
    ]

    let {
        isFiltersOpen = $bindable(),
        showWatchedMedia = $bindable(),
        showHiddenMedia = $bindable(),
        type = $bindable(),
        selectedRole = $bindable(),
    } = $props()

    let newShowWatched = $state(showWatchedMedia)
    let newShowHidden = $state(showHiddenMedia)
    let newType = $state(type)
    let newRole = $state(selectedRole)

    const onCancel = (): void => {
        isFiltersOpen = false
    }

    const resetFilters = (): void => {
        newShowWatched = true
        newShowHidden = true
        newType = 'all types'
        newRole = 'all roles'
    }

    const setFilters = (): void => {
        showWatchedMedia = newShowWatched
        showHiddenMedia = newShowHidden
        type = newType
        selectedRole = newRole
        isFiltersOpen = false
    }
</script>

<BaseModal onClose={onCancel}>
    <div class="flex w-full flex-col rounded-lg p-4" tabindex="-1">
        <h1 class="flex w-full text-xl">
            Filters <button onclick={resetFilters} class="ml-auto flex">
                <ResetIcon class="text-sm text-slate-500 hover:text-primaryColor" />
            </button>
        </h1>
        <div class="flex justify-center gap-8 p-8">
            <div class="flex w-36 flex-col gap-4">
                <Select maxWidth="w-full" options={roles} bind:activeOption={newRole} func={() => {}} name="Roles" />
                <Checkbox label="Show Watched" bind:checked={newShowWatched} func={() => {}} />
            </div>
            <div class="mx-auto flex w-36 flex-col justify-center gap-4">
                <Select maxWidth="w-full" options={types} bind:activeOption={newType} func={() => {}} name="Type" />
                <Checkbox label="Show Hidden" bind:checked={newShowHidden} func={() => {}} />
            </div>
        </div>
        <div class="ml-auto flex gap-4">
            <button
                onclick={onCancel}
                class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">Cancel</button>
            <button
                onclick={setFilters}
                class="flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-white shadow-lg transition-all hover:text-primaryColor"
                >Accept</button>
        </div>
    </div>
</BaseModal>
