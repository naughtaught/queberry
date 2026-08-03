<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { createError, handleError } from '$lib/functions/errors/errorHandling'
    import { appData, toastNotification } from '$lib/stores/app'
    import { user } from '$lib/stores/user'
    import { Reports } from '$lib/types/reports'
    import EmailInputField from '$lib/components/inputs/EmailInputField.svelte'
    import Select from '$lib/components/inputs/Select.svelte'
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { page } from '$app/state'

    let { isReportModalOpen = $bindable(), media } = $props()

    let description = $state('')
    const suggestedFix = $state('')
    let email = $state($user?.email || null)
    let contentIssue = $state(Reports.ContentType.INACCURACY)

    const isValid = $derived(description.length > 0 && contentIssue && media && $user)

    const handleSubmit = async (): Promise<void> => {
        if (!$user) throw createError('Missing User', 401, { log: false })

        try {
            const resp = await invokeFunction('api_submit_report', {
                postgresId: $user.postgresId,
                token: $user.token,
                email: email || $user.email || null,
                params: {
                    reportType: 'content',
                    description,
                    severity: null,
                    stepsToReproduce: null,
                    logFile: null,
                    contentType: contentIssue,
                    contentId: media.id,
                    contentLocation: page.route.id,
                    suggestedFix,
                    appVersion: $appData.currentVersion,
                    email: email || $user.email || null,
                },
            })

            if (!resp.success) throw resp.error

            toastNotification.show({
                title: 'Success!',
                message: 'Content report successfully submitted.',
                type: 'success',
            })
        } catch (error) {
            handleError(error)
        } finally {
            isReportModalOpen = false
        }
    }

    const onCancel = (): void => {
        isReportModalOpen = false
    }
</script>

<BaseModal onClose={onCancel}>
    <div class="w-full transform rounded-xl p-6" tabindex="-1" id="report-modal">
        <h2 id="modal-title" class="mb-6 text-xl font-semibold text-textColor">Bug Report</h2>

        <div class="min-w-[50vw] space-y-5">
            <div>
                <div class="mb-2">
                    <EmailInputField bind:email />
                </div>

                <label class="mb-2 text-xs font-semibold tracking-wider text-slate-400 uppercase">
                    Description <span class="text-red-500">*</span>
                    <textarea
                        id="description"
                        bind:value={description}
                        rows="4"
                        class="w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-3 text-textColor placeholder-slate-500"
                        placeholder="Describe the issue."
                        maxlength="5000"></textarea>
                    <p class="mt-1 text-xs text-slate-500">{description.length}/5000</p>
                </label>
            </div>
            <div class="mb-3">
                <Select
                    maxWidth="w-full"
                    labelOptions="text-xs font-semibold tracking-wider text-slate-400 uppercase"
                    options={Reports.ContentTypes}
                    bind:activeOption={contentIssue}
                    name="Content Issue" />
            </div>
        </div>

        <div class="mt-6 flex justify-end space-x-4">
            <button
                onclick={onCancel}
                class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
                Cancel
            </button>
            <button
                onclick={handleSubmit}
                disabled={!isValid}
                class="rounded-lg px-6 py-3.5 font-bold text-textColor shadow-lg transition-all hover:text-primaryColor disabled:cursor-default disabled:opacity-50 disabled:hover:text-textColor {isValid
                    ? 'bg-slate-800 hover:text-primaryColor'
                    : 'bg-slate-800 text-slate-500'}">
                Submit Report
            </button>
        </div>
    </div>
</BaseModal>
