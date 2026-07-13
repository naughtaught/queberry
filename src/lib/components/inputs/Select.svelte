<script lang="ts">
    import { clickOutside } from '$lib/functions/utility/useClickOutside'
    import SortIcon from 'virtual:icons/material-symbols/sort'

    let {
        options,
        name,
        activeOption = $bindable(),
        ascending = $bindable(null),
        maxWidth = 'max-w-1/2',
        func = () => {},
    } = $props()

    let isOpen = $state(false)
    const displayName = $derived.by(() => {
        const option = options.find((x: { value: string | number }) => x.value === activeOption)
        return option?.name || ''
    })

    const toggleDropdown = (): void => {
        isOpen = !isOpen
    }

    const handleClick = (item: { name: string; value: string | number }): void => {
        activeOption = item.value
        isOpen = false
        func()
    }
</script>

<div
    class="w-full {maxWidth} shrink-0 grow-0"
    use:clickOutside
    onclickOutside={() => {
        isOpen = false
    }}>
    <div class="mb-1 flex w-full items-center justify-between text-sm tracking-wide text-slate-500">
        <span>{name}</span>
        <button
            class=" hover:text-primaryColor"
            onclick={() => {
                ascending = !ascending
            }}
            >{#if ascending !== null}<SortIcon class={ascending ? '' : 'rotate-180'} />{/if}</button>
    </div>
    <div class="relative z-30 flex min-h-8.5 w-full flex-col" class:z-30={isOpen} class:z-10={!isOpen}>
        <button
            type="button"
            class="flex min-h-8.5 w-full min-w-30 items-center justify-between rounded-t border-x border-t border-b border-slate-700 bg-slate-800/50 px-2 py-1 text-left"
            class:border-b-transparent={isOpen}
            class:rounded-b={!isOpen}
            onclick={toggleDropdown}>
            <span class="pointer-events-none min-w-0 flex-1 truncate">
                {displayName}
            </span>
            <span class="pointer-events-none ml-2 shrink-0 text-xs text-slate-300">
                {isOpen ? '▲' : '▼'}
            </span>
        </button>
        {#if isOpen}
            <div
                class="absolute top-full right-0 left-0 z-30 max-h-60 overflow-y-auto rounded-b border-x border-b border-slate-700 bg-slate-800">
                {#each options as item, index (index)}
                    <button
                        type="button"
                        class="flex w-full items-center p-2 text-left text-sm font-semibold tracking-wider text-slate-300 uppercase hover:text-primaryColor"
                        onclick={() => handleClick(item)}>
                        {item.name}
                    </button>
                {/each}
            </div>
        {/if}
    </div>
</div>
