<script lang="ts">
    import { modals } from '$lib/stores/app'
    import CloseIcon from 'virtual:icons/material-symbols/close'
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { settings } from '$lib/stores/user'

    const { media, func = () => {} } = $props()

    const handleClose = (): void => {
        func()
        $modals.trailer = false
    }

    let iframeEl: HTMLIFrameElement

    const setVolume = (): void => {
        if (!iframeEl?.contentWindow) return

        try {
            iframeEl.contentWindow.postMessage(
                JSON.stringify({
                    event: 'command',
                    func: 'setVolume',
                    args: [$settings.trailerVolume],
                }),
                'https://www.youtube.com',
            )
        } catch (error) {
            handleError(error)
        }
    }

    const onIframeLoad = (): void => {
        setTimeout(setVolume, 500)
    }
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
                <iframe
                    bind:this={iframeEl}
                    class="absolute top-0 left-0 h-full w-full border-0"
                    loading="lazy"
                    title={media.title}
                    src={`https://youtube.com/embed/${media?.trailer}?autoplay=1&enablejsapi=1&rel=0`}
                    frameborder="0"
                    allowfullscreen
                    onload={onIframeLoad}></iframe>
            </div>
        </div>
    </div>
</BaseModal>
