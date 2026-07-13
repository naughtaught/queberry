<script lang="ts">
    import { modals, primaryUser } from '$lib/stores/app'

    import EmailInputField from '$lib/components/inputs/EmailInputField.svelte'
    import PinInputField from '$lib/components/inputs/PinInputField.svelte'
    import PasswordInputField from '$lib/components/inputs/PasswordInputField.svelte'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { user } from '$lib/stores/user'
    import AvatarSelection from '$lib/components/inputs/AvatarSelection.svelte'
    import { validateLoginForm } from '$lib/functions/utility/validateLoginForm'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { updateGlobalSettings } from '$lib/functions/user/updateGlobalSettings'
    import { loginUser } from '$lib/functions/user/loginUser'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import UsernameInputField from '$lib/components/inputs/UsernameInputField.svelte'

    let selectedAvatar: string | null = $state(null)
    let email: string | null = $state(null)
    let password: string | null = $state(null)
    let passwordConfirmation: string | null = $state(null)
    let username: string | null = $state(null)
    let errorMessage: string | null = $state(null)
    let pin: string | null = $state(null)
    let parentalControls = $derived(false)
    let primaryAccount = $derived($primaryUser === null || $primaryUser === undefined)
    let isLoading = $state(false)

    const createUser = async (): Promise<void> => {
        if (isLoading) return
        isLoading = true
        if (!selectedAvatar) {
            errorMessage = 'Please select an Avatar.'
            return
        }

        try {
            if (email && password && passwordConfirmation && selectedAvatar && pin && username) {
                const isValidForm = validateLoginForm(email, password, pin, passwordConfirmation, username)
                if (isValidForm.error) {
                    errorMessage = isValidForm.error
                    return
                }

                const authResponse = await invokeFunction('api_register', { email, username, password })
                if (!authResponse.success) {
                    errorMessage = authResponse.error.message
                    return
                }

                const response = await invokeFunction('create_user', {
                    user: {
                        username: authResponse.data.username,
                        email,
                        avatar: selectedAvatar,
                        pin,
                        postgresId: authResponse.data.postgresId,
                        token: authResponse.data.token,
                    },
                })

                if (!response.success) throw response.error

                if (primaryAccount) {
                    const resp = await updateGlobalSettings(response.data, parentalControls)
                    if (!resp.success) throw resp.error
                }

                loginUser(response.data, true)
            }
        } catch (error) {
            handleError(error)
        } finally {
            isLoading = false
        }
    }

    $effect(() => {
        if (errorMessage) {
            const timeout = setTimeout(() => {
                errorMessage = null
            }, 5000)
            return () => clearTimeout(timeout)
        }
    })
</script>

<div class="flex flex-row overflow-y-auto">
    <form class="border-r border-slate-700/30 px-6 pt-6 pb-4">
        <div class="mb-8">
            <div class="mb-2 flex items-center gap-3">
                <span class="text-3xl"> <img class="h-8 w-8" src="/images/logo.png" alt="logo" /></span>
                <h2 class="text-2xl font-bold tracking-tight text-slate-100">Queberry</h2>
            </div>
            <p class="text-slate-400">Create a new account</p>
        </div>
        <div class="flex gap-5">
            <div class="flex flex-col gap-5">
                <UsernameInputField bind:username />
                <EmailInputField bind:email />
                <PinInputField bind:pin />
            </div>
            <div class="flex flex-col gap-5">
                <PasswordInputField name="Password" bind:password />
                <PasswordInputField name="Confirm Password" bind:password={passwordConfirmation} />
                <div class="flex flex-1 flex-col justify-end gap-3 pb-1">
                    <Checkbox
                        label="Primary Account?"
                        bind:checked={primaryAccount}
                        disabled={$primaryUser !== null && $primaryUser !== undefined} />
                    <Checkbox
                        label="Enable Parental Controls?"
                        bind:checked={parentalControls}
                        disabled={!primaryAccount} />
                </div>
            </div>
        </div>
        <div class="mt-10 flex gap-4">
            <button
                onclick={createUser}
                class="flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-white shadow-lg transition-all hover:text-primaryColor">
                {#if !isLoading}Register Account{:else}<LoadingSpinner />
                {/if}
            </button>
            <button
                disabled={!$user}
                onclick={() => {
                    if (!isLoading) return
                    $modals.user = false
                }}
                class="{$user
                    ? 'text-slate-400 hover:text-slate-200'
                    : 'cursor-default! text-neutral-700'} px-6 py-3.5 font-medium transition-colors">
                Cancel
            </button>
        </div>
        <p class="mt-4 h-4 text-red-500">{errorMessage ? errorMessage : ''}</p>
    </form>
    <AvatarSelection bind:selectedAvatar {isLoading} />
</div>
