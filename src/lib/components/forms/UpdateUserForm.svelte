<script lang="ts">
    import BaseModal from '$lib/components/modals/BaseModal.svelte'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { clickOutside } from '$lib/functions/utility/useClickOutside'
    import { user, users } from '$lib/stores/user'
    import AvatarSelection from '$lib/components/inputs/AvatarSelection.svelte'
    import EmailInputField from '$lib/components/inputs/EmailInputField.svelte'
    import PasswordInputField from '$lib/components/inputs/PasswordInputField.svelte'
    import PinInputField from '$lib/components/inputs/PinInputField.svelte'
    import { validatePassword } from '$lib/functions/utility/validatePassword'
    import { validateUsername } from '$lib/functions/utility/validateUsername'
    import { validateEmail } from '$lib/functions/utility/validateEmail'
    import { validatePin } from '$lib/functions/utility/validatePin'
    import type { Api } from '$lib/types/Api'
    import type { Sql } from '$lib/types/sql'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import UsernameInputField from '$lib/components/inputs/UsernameInputField.svelte'

    let { isUpdateModalOpen = $bindable() } = $props()

    let selectedAvatar = $state(null)
    let username = $state(null)
    let email = $state(null)
    let oldPassword = $state(null)
    let newPassword = $state(null)
    let oldPin = $state(null)
    let newPin = $state(null)
    let errorMessage: string | null = $state(null)
    let isLoading = $state(false)

    const updateAccount = async (): Promise<void> => {
        if (!$user) return

        isLoading = true

        try {
            if (!email && !oldPin && !oldPassword && !selectedAvatar && !username) return

            const sqliteUpdate: Sql.UserUpdate = {
                userId: $user.id,
                token: $user.token,
            }
            const dbUpdate: Api.UserUpdate = {}

            if (oldPassword && newPassword) {
                const isPasswordValid = validatePassword(newPassword)
                if (!isPasswordValid.success) {
                    errorMessage = isPasswordValid.error
                    return
                }

                dbUpdate.current_password = oldPassword
                dbUpdate.new_password = newPassword
            }

            if (username) {
                const isUsernameValid = validateUsername(username)
                if (!isUsernameValid.success) {
                    errorMessage = isUsernameValid.error
                    return
                }
                sqliteUpdate.username = username
                dbUpdate.username = username
            }

            if (email) {
                const isEmailValid = validateEmail(email)
                if (!isEmailValid.success) {
                    errorMessage = isEmailValid.error
                    return
                }
                sqliteUpdate.email = email
                dbUpdate.email = email
            }

            if (newPin && oldPin) {
                const response = await invokeFunction('verify_pin', {
                    userId: $user.id,
                    pin: oldPin,
                })

                if (!response.success) throw response.error

                if (!response.data) {
                    errorMessage = 'Incorrect PIN.'
                    return
                }

                const isPinValid = validatePin(newPin)
                if (!isPinValid.success) {
                    errorMessage = isPinValid.error
                    return
                }
                sqliteUpdate.pin = newPin
            }

            if (selectedAvatar) sqliteUpdate.avatar = selectedAvatar

            if (Object.keys(dbUpdate).length > 0) {
                const apiUpdate = await invokeFunction('api_update_user', {
                    postgresId: $user.postgresId,
                    token: $user.token,
                    email: dbUpdate.email || null,
                    username: dbUpdate.username || null,
                    currentPassword: dbUpdate.current_password || null,
                    newPassword: dbUpdate.new_password || null,
                })

                if (!apiUpdate.success) {
                    if (apiUpdate.error.message === 'Runtime error: Current password is incorrect') {
                        errorMessage = 'Password is incorrect'
                        return
                    }
                    throw apiUpdate.error || 'Failed to update server profile'
                }
            }

            if (Object.keys(sqliteUpdate).length > 0) {
                const sqlUpdate = await invokeFunction('update_user', {
                    updates: sqliteUpdate,
                })

                if (!sqlUpdate.success) throw sqlUpdate.error || 'Failed to update local profile'

                $user = sqlUpdate.data

                const allUsers = await invokeFunction('get_all_users', {})
                if (!allUsers.success) throw allUsers.error

                $users = allUsers.data
            }
            isUpdateModalOpen = false
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

<BaseModal onClose="">
    <div
        use:clickOutside
        onclickOutside={() => {
            isUpdateModalOpen = false
        }}
        class="flex h-129 min-w-220 flex-row overflow-y-auto">
        <form class="flex flex-1 flex-col border-r border-slate-700/30 px-6 pt-6 pb-4">
            <div class="mb-8">
                <div class="mb-2 flex items-center gap-3">
                    <span class="text-3xl"> <img class="h-8 w-8" src="/images/logo.png" alt="logo" /></span>
                    <h2 class="text-2xl font-bold tracking-tight text-slate-100">Queberry</h2>
                </div>
                <p class="text-slate-400">Update account</p>
            </div>
            <div class="flex gap-5">
                <div class="flex flex-col gap-5">
                    <EmailInputField bind:email required={false} />
                    <PinInputField bind:pin={oldPin} labelText="Old Pin" required={false} />
                    <PinInputField bind:pin={newPin} labelText="New Pin" required={false} />
                </div>
                <div class="flex flex-col gap-5">
                    <UsernameInputField bind:username required={true} />
                    <PasswordInputField name="Old Password" bind:password={oldPassword} required={false} />
                    <PasswordInputField name="New Password" bind:password={newPassword} required={false} />
                </div>
            </div>
            <div class="flex-1"></div>
            <div class="mt-10 flex gap-4">
                <button
                    disabled={isLoading}
                    onclick={updateAccount}
                    class="flex-1 rounded-lg bg-slate-800 px-6 py-3.5 font-bold text-white shadow-lg transition-all hover:text-primaryColor">
                    {#if !isLoading}Update{:else}<LoadingSpinner />{/if}
                </button>
                <button
                    disabled={isLoading}
                    onclick={() => {
                        isUpdateModalOpen = false
                    }}
                    class="px-6 py-3.5 font-medium text-slate-400 transition-colors hover:text-slate-200">
                    Cancel
                </button>
            </div>
            <p class="mt-4 h-4 text-red-500">{errorMessage ? errorMessage : ''}</p>
        </form>
        <AvatarSelection bind:selectedAvatar {isLoading} />
    </div>
</BaseModal>
