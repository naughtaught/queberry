<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { loginUser } from '$lib/functions/user/loginUser'
    import { getAvatar } from '$lib/functions/utility/getAvatar'
    import { validatePin } from '$lib/functions/utility/validatePin'
    import { parentalControlsAreEnabled } from '$lib/stores/app'
    import { users } from '$lib/stores/user'
    import type { Sql } from '$lib/types/sql'

    import { onMount } from 'svelte'
    import PinInputField from '$lib/components/inputs/PinInputField.svelte'

    const pins: Record<string, string | null> = $state({})
    let errorMessage: string | null = $state(null)
    const avatars: string[] = $state([])

    onMount(() => {
        $users.forEach(async (user) => {
            const avatar = await getAvatar(user.avatar)
            avatars.push(avatar)
        })
    })

    const handleClick = async (user: Sql.User): Promise<void> => {
        try {
            if ($parentalControlsAreEnabled) {
                const pin = pins[user.id]
                if (!pin) {
                    errorMessage = 'PIN is required.'
                    return
                }

                const isPinValid = validatePin(pin)
                if (isPinValid.error) {
                    errorMessage = isPinValid.error
                    return
                }

                const response = await invokeFunction('verify_pin', {
                    userId: user.id,
                    pin,
                })

                if (!response.success) throw response.error

                if (!response.data) {
                    errorMessage = 'Incorrect PIN.'
                    return
                }

                if (response.data) loginUser(user)
            } else {
                loginUser(user)
            }
        } catch (error) {
            handleError(error)
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

<div class="flex min-w-200 flex-col">
    <div class="flex-1 px-6 pt-6 pb-4">
        <div class="mb-8">
            <div class="mb-2 flex items-center gap-3">
                <span class="text-3xl"> <img class="h-8 w-8" src="/images/logo.png" alt="logo" /></span>
                <h2 class="text-2xl font-bold tracking-tight text-slate-100">Queberry</h2>
            </div>
            <p class="text-slate-400">Log in to your account</p>
        </div>
        <div class="mx-auto flex min-w-62.5 flex-wrap justify-center">
            <div class="my-auto flex flex-wrap gap-x-8 gap-y-3">
                {#each $users as user, index (user.id)}
                    <div class="group flex flex-col items-center space-y-2">
                        <button
                            onclick={() => {
                                handleClick(user)
                            }}
                            class="flex shrink-0 flex-col items-center justify-center rounded text-center text-xs hover:cursor-pointer">
                            <img
                                class="size-30 rounded-xl border-3 border-textColor/10 bg-cover bg-center group-hover:border-primaryColor"
                                src={avatars[index]}
                                alt="User Avatar" />
                            {user.username}
                        </button>
                        {#if $parentalControlsAreEnabled}
                            <div class="flex w-34 flex-col">
                                <PinInputField
                                    bind:pin={pins[user.id]}
                                    label={false}
                                    height="h-8"
                                    isUserSelect={true}
                                    func={() => handleClick(user)} />
                                <button
                                    onclick={() => {
                                        handleClick(user)
                                    }}
                                    class="mt-2 flex-1 rounded-lg bg-slate-800 px-3 py-1 font-bold text-white shadow-lg transition-all hover:text-primaryColor"
                                    >Submit</button>
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    </div>
    <p class="mt-4 h-4 text-center text-red-500">{errorMessage ? errorMessage : ''}</p>
</div>
