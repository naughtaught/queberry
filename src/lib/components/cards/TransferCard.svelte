<script lang="ts">
    import { deleteTransfer } from '$lib/functions/plugins/deleteTransfer'
    import CloseIcon from 'virtual:icons/material-symbols/close'

    const { transfer } = $props()
</script>

<div
    class="glass-panel group sidebar-gradient relative w-full overflow-hidden rounded-lg p-8 shadow-2xl transition-all duration-500">
    <div
        class="absolute top-0 left-0 h-full w-1 origin-top scale-y-0 bg-primaryColor transition-transform group-hover:scale-y-100">
    </div>
    <div class="flex items-center gap-8">
        <div class="grow">
            <div class="mb-4 flex items-end justify-between">
                <div>
                    <h3 class="mb-1 text-xl font-bold">{transfer.filename}</h3>
                    <div class="flex items-center gap-4">
                        <span class="flex items-center gap-1 text-[10px] tracking-[0.2em] uppercase">
                            <span class="text-[14px]">{transfer.status}</span>
                        </span>
                        <span class="flex items-center gap-1 text-[10px] tracking-[0.2em] text-textColor uppercase">
                            {transfer.speed} Mbps
                        </span>
                    </div>
                </div>
                <div class="text-right">
                    <span class="text-2xl font-black tracking-tighter">{transfer.progress}%</span>
                </div>
            </div>
            <div class="h-2 w-full overflow-hidden rounded-full bg-white/5">
                <div class="h-full bg-primaryColor transition-all duration-1000" style="width: 56%"></div>
            </div>
        </div>
        <div class="flex gap-2">
            <button
                onclick={() => deleteTransfer(transfer.info_hash)}
                class="group/closebutton flex h-12 w-12 items-center justify-center rounded-full bg-white/5 transition-colors hover:bg-red-500/20">
                <CloseIcon class="text-textColor transition-colors group-hover/closebutton:text-red-500" />
            </button>
        </div>
    </div>
</div>
