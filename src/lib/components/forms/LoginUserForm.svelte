<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { loginUser } from '$lib/functions/user/loginUser'
    import { validateLoginForm } from '$lib/functions/utility/validateLoginForm'
    import { modals, primaryUser } from '$lib/stores/app'
    import { user } from '$lib/stores/user'
    import AvatarSelection from '$lib/components/inputs/AvatarSelection.svelte'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
    import EmailInputField from '$lib/components/inputs/EmailInputField.svelte'
    import PasswordInputField from '$lib/components/inputs/PasswordInputField.svelte'
    import PinInputField from '$lib/components/inputs/PinInputField.svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { updateGlobalSettings } from '$lib/functions/user/updateGlobalSettings'

    let selectedAvatar = $state(null)
    let email = $state(null)
    let password = $state(null)
    let errorMessage: string | null = $state(null)
    let parentalControls = $derived(false)
    let primaryAccount = $derived($primaryUser === null || $primaryUser === undefined)
    let pin: string | null = $state(null)
    let isLoading = $state(false)

    const createLocalAccount = async (): Promise<void> => {
        if (isLoading) return
        isLoading = true

        try {
            if (!selectedAvatar) {
                errorMessage = 'Please select an Avatar.'
                return
            }
            if (email && password && selectedAvatar && pin) {
                const isValidForm = validateLoginForm(email, password, pin)
                if (isValidForm.error) {
                    errorMessage = isValidForm.error
                    return
                }

                const authResponse = await invokeFunction('api_login', { email, password })
                if (!authResponse.success) {
                    errorMessage = 'Invalid Credentials'
                    return
                }

                const existingUser = await invokeFunction('get_user_by_postgres_id', {
                    postgresId: authResponse.data.postgresId,
                })

                if (!existingUser.success) throw existingUser.error

                if (existingUser.data) {
                    const updateUser = await invokeFunction('update_user', {
                        updates: {
                            userId: existingUser.data.id,
                            token: authResponse.data.token,
                        },
                    })
                    if (!updateUser.success) throw updateUser.error

                    loginUser(updateUser.data, true)
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
                    if (!resp.success) handleError(resp.error)
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

<div class="flex h-129 min-w-220 flex-row overflow-y-auto">
    <form class="flex flex-1 flex-col border-r border-slate-700/30 px-6 pt-6 pb-4">
        <div class="mb-8">
            <div class="mb-2 flex items-center gap-3">
                <span class="text-3xl"> <img class="h-8 w-8" src="/images/logo.png" alt="logo" /></span>
                <h2 class="text-2xl font-bold tracking-tight text-slate-100">Queberry</h2>
            </div>
            <p class="text-slate-400">Add an existing account</p>
        </div>
        <div class="flex gap-5">
            <div class="flex flex-col gap-5">
                <EmailInputField bind:email />
                <PinInputField bind:pin />
            </div>
            <div class="flex flex-col gap-5">
                <PasswordInputField name="Password" bind:password />
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
        <div class="flex-1"></div>
        <div class="mt-10 flex gap-4">
            <button
                disabled={!$user}
                onclick={() => {
                    if (isLoading) return
                    $modals.user = false
                }}
                class="{$user
                    ? 'text-slate-400 hover:text-slate-200'
                    : 'cursor-default! text-neutral-700'} px-6 py-3.5 font-medium transition-colors">
                Cancel
            </button>
            <button
                onclick={createLocalAccount}
                class="flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-white shadow-lg transition-all hover:text-primaryColor">
                {#if !isLoading}Login{:else}<LoadingSpinner />
                {/if}
            </button>
        </div>
        <p class="mt-4 h-4 text-red-500">{errorMessage ? errorMessage : ''}</p>
    </form>
    <AvatarSelection bind:selectedAvatar {isLoading} />
</div>
