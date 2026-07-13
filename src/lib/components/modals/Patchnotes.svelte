<script lang="ts">
    import CloseIcon from 'virtual:icons/material-symbols/close'
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import Markdown from 'svelte-markdown'
    import { appData } from '$lib/stores/app'
    import type { App } from '$lib/types/app'

    let { isPatchNotesModalOpen = $bindable() } = $props()

    const handleClose = (): void => {
        isPatchNotesModalOpen = false
    }

    const formatPatchNotes = (notes: App.PatchNotes | null): string => {
        if (typeof notes === 'string') return notes
        if (!notes) return ''

        let markdown = `## ${notes.title || 'Patch Notes'}\n\n`

        if (notes.features?.length) {
            markdown += `### New Features\n`
            notes.features.forEach((f: string) => (markdown += `- ${f}\n`))
            markdown += `\n`
        }

        if (notes.fixes?.length) {
            markdown += `### Bug Fixes\n`
            notes.fixes.forEach((f: string) => (markdown += `- ${f}\n`))
            markdown += `\n`
        }

        if (notes.known_issues?.length) {
            markdown += `### Known Issues\n`
            notes.known_issues.forEach((i: string) => (markdown += `- ${i}\n`))
            markdown += `\n`
        }

        return markdown
    }

    const patchNotes = $derived(formatPatchNotes($appData.currentNotes))
</script>

<BaseModal onClose={handleClose}>
    <div class="pointer-events-auto relative flex items-center justify-center">
        <div class="relative" style="width: min(80vw, calc(80vh * 16/9)); max-width: 90vw;">
            <div class="relative aspect-video w-full">
                <button
                    tabindex="0"
                    class="absolute -top-5 -right-5 z-50 rounded-full bg-black/50 p-2 text-textColor hover:text-primaryColor"
                    onclick={handleClose}>
                    <CloseIcon class="h-6 w-6" />
                </button>
                <div class="h-full w-full overflow-y-auto p-8">
                    {#if $appData.currentNotes}
                        <div class="prose max-w-none prose-invert">
                            <Markdown source={patchNotes} />
                        </div>
                    {:else}
                        <p class="text-textColor/60">No patch notes available for this version.</p>
                    {/if}
                </div>
            </div>
        </div>
    </div>
</BaseModal>
