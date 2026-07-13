<script lang="ts">
    import CancelIcon from 'virtual:icons/healthicons/cancel-24px'
    import LinkIcon from 'virtual:icons/material-symbols/link'

    import { resolve } from '$app/paths'
    import { getImagePath } from '$lib/functions/ui/getImagePath'
    import { posterSize } from '$lib/functions/ui/posterSize'

    const { request, cancelRequest } = $props()

    const posterPath = $derived(getImagePath(request?.poster, posterSize(1, 'carousel')))
    const statusColor = (type: 'bg' | 'text'): string => {
        switch (request.status) {
            case 'pending':
                return `${type}-amber-500`
            case 'processing':
                return `${type}-blue-500`
            case 'failed':
                return `${type}-red-500`
            case 'completed':
                return `${type}-green-500`
            default:
                return ''
        }
    }
</script>

<div class="grid grid-cols-12 items-center border-t border-white/5 px-8 py-6 transition-colors hover:bg-white/5">
    <div class="col-span-4 flex items-center gap-4">
        <div class="h-16 w-12 shrink-0 overflow-hidden rounded-md">
            <img
                alt="Movie Poster"
                class="h-full w-full object-cover"
                src={posterPath ? posterPath : '/images/poster-placeholder.png'} />
        </div>
        <div>
            <h3 class="mb-1 truncate leading-tight font-bold">
                {#if request.title}
                    {request.title}
                    {#if request.released}({request.released}){/if}
                {/if}
            </h3>
            <p class="inline-block rounded bg-primaryColor/10 px-2 py-0.5 font-mono text-xs text-textColor">
                {request.imdb_id}
            </p>
        </div>
    </div>
    <div class="col-span-2">
        <div class="flex items-center gap-2">
            <span class="h-2 w-2 animate-pulse rounded-full {statusColor('bg')}"></span>
            <span class="text-sm font-semibold capitalize {statusColor('text')}">{request.status}</span>
        </div>
    </div>
    <div class="col-span-1 text-center">
        <div class="flex flex-col gap-1">
            <span class="text-[11px] text-slate-200">{request.tmdb_id}</span>
        </div>
    </div>
    <div class="col-span-1 text-center">
        <div class="flex flex-col gap-1">
            <span class="text-[11px] text-slate-200">{request.tvdb_id}</span>
        </div>
    </div>
    <div class="col-span-2 text-right">
        <span class="text-sm text-slate-400">{new Date(request.created_at).toLocaleDateString()}</span>
    </div>
    <div class="col-span-2 text-right">
        {#if request.status !== 'completed'}
            <button
                onclick={() => cancelRequest?.(request.id)}
                class="ml-auto flex items-center justify-end gap-1 text-xs font-bold text-slate-500 transition-colors">
                Cancel
                <CancelIcon class="text-xs text-red-500" />
            </button>
        {:else}
            <a
                class="ml-auto flex items-center justify-end gap-1 text-sm font-bold text-slate-500 transition-colors"
                data-sveltekit-preload-data="off"
                tabindex="-1"
                href={resolve(`/details/?id=${request.media_id}&type=${request.media_type}`, {})}>
                View
                <LinkIcon class="float-right text-xs text-white" />
            </a>
        {/if}
    </div>
</div>
