<script lang="ts">
    import { clickOutside } from "$lib/functions/utility/useClickOutside"


    let { dataArr, name, includeData = $bindable([]), excludeData = $bindable([]) } = $props()

    let isOpen = $state(false)
    let search = $state('')

    const filteredArr = $derived.by(() => {
        if (!search) return dataArr
        return dataArr.filter((x: string) => x.toLowerCase().includes(search.toLowerCase()))
    })

    const toggleDropdown = (): void => {
        isOpen = !isOpen
        if (!isOpen) search = ''
    }

    const handleClick = (item: string): void => {
        if (!includeData.includes(item) && !excludeData.includes(item)) {
            includeData = [...includeData, item]
        } else if (includeData.includes(item)) {
            includeData = includeData.filter((x) => x !== item)
            excludeData = [...excludeData, item]
        } else if (excludeData.includes(item)) {
            excludeData = excludeData.filter((x) => x !== item)
        }
    }

    const clearAll = (): void => {
        includeData = []
        search = ''
    }
</script>

<div
    class="w-full max-w-1/2 shrink-0 grow-0"
    use:clickOutside
    onclickOutside={() => {
        isOpen = false
        search = ''
    }}>
    <div class="mb-1 flex text-sm tracking-wide text-slate-500">
        <span>{name}</span>
    </div>
    <div class="relative z-30 flex min-h-8.5 w-full flex-col" class:z-30={isOpen} class:z-10={!isOpen}>
        <button
            type="button"
            class="flex min-h-8.5 w-full min-w-30 items-center justify-between rounded-t border-x border-t border-slate-700 bg-slate-800/50 px-2 py-1"
            class:border-b={!isOpen}
            class:rounded-b={!isOpen}
            onclick={toggleDropdown}>
            <span class="pointer-events-none max-w-full truncate">
                <span class="pointer-events-none min-w-0 flex-1 truncate">
                    {#each includeData as item, i (i)}
                        <span>{item.trim()}{i < includeData.length - 1 || excludeData.length > 0 ? ', ' : ''} </span>
                    {/each}
                    {#each excludeData as item, i (i)}
                        <span class="text-red-500">{item.trim()}{i < excludeData.length - 1 ? ', ' : ''} </span>
                    {/each}
                </span>
            </span>
            <span class="pointer-events-none ml-2 shrink-0 text-xs text-slate-300">
                {isOpen ? '▲' : '▼'}
            </span>
        </button>

        {#if isOpen}
            <div
                class="absolute top-full right-0 left-0 z-30 max-h-60 min-h-8.5 overflow-y-auto rounded-b border-x border-b border-slate-700 bg-slate-800">
                <div class="relative mx-auto my-1 flex bg-red-500">
                    <input
                        class="w-full border-0 bg-slate-800 px-2 py-1 text-sm text-slate-300"
                        placeholder="Search..."
                        bind:value={search}
                        type="text" />
                    {#if search}
                        <button
                            type="button"
                            class="absolute top-0 right-0 bottom-0 flex items-center justify-center px-2 text-red-600 hover:text-red-700"
                            onclick={(event) => {
                                event.stopPropagation()
                                search = ''
                            }}>
                            ✗
                        </button>
                    {/if}
                </div>

                <button
                    type="button"
                    class="flex w-full p-2 text-left font-semibold tracking-wider uppercase"
                    onclick={clearAll}>
                    <span class="text-sm text-slate-300 hover:text-primaryColor">Clear All</span>
                </button>

                {#each filteredArr as item, index (index)}
                    <button
                        type="button"
                        class="flex w-full items-center p-2 text-left text-sm font-semibold tracking-wider text-slate-300 uppercase hover:text-primaryColor"
                        onclick={() => handleClick(item)}>
                        <div class="relative mr-2 flex items-center">
                            {#if includeData.includes(item)}
                                <div
                                    class="flex h-5 w-5 items-center justify-center rounded border border-slate-700 bg-primaryColor text-black shadow">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-3.5 w-3.5"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        stroke="currentColor"
                                        stroke-width="1">
                                        <path
                                            fill-rule="evenodd"
                                            d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                            clip-rule="evenodd"></path>
                                    </svg>
                                </div>
                            {:else if excludeData.includes(item)}
                                <div
                                    class="flex h-5 w-5 items-center justify-center rounded border border-gray-900 bg-red-100 shadow">
                                    <span class="font-bold text-red-600">✗</span>
                                </div>
                            {:else}
                                <div class="h-5 w-5 rounded border border-gray-400 shadow"></div>
                            {/if}
                        </div>
                        {item}
                    </button>
                {/each}
            </div>
        {/if}
    </div>
</div>
