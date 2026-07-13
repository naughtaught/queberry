<script lang="ts">
    import LoginUserForm from '$lib/components/forms/LoginUserForm.svelte'
    import { modals } from '$lib/stores/app'
    import { users } from '$lib/stores/user'
    import RegisterUserForm from '$lib/components/forms/RegisterUserForm.svelte'
    import UserSelect from '$lib/components/inputs/UserSelect.svelte'
    import BaseModal from '$lib/components/modals/BaseModal.svelte'

    let activePage = $derived($users && $users.length > 0 ? 'Users' : 'Register')

    const handleClose = (): void => {
        $modals.user = false
    }
</script>

{#snippet pageButton(text: string)}
    <button
        onclick={() => (activePage = text)}
        class="group flex items-center gap-2 rounded-xl px-4 py-3 text-textColor transition-all">
        <span
            class="relative flex cursor-pointer items-center pr-1 text-sm font-semibold tracking-wide after:absolute after:-bottom-[1.5px] after:left-0 after:h-0.5 after:bg-primaryColor after:transition-all after:duration-100 group-hover:after:w-full"
            class:after:w-full={activePage === text}>
            {text}
        </span>
    </button>
{/snippet}

<BaseModal onClose={handleClose}>
    {#if activePage === 'Users'}
        <UserSelect />
    {:else if activePage === 'Login'}
        <LoginUserForm />
    {:else}
        <RegisterUserForm />
    {/if}
    <div class="flex w-full flex-col rounded-b-xl shadow-2xl">
        <div class="flex justify-center p-3">
            <nav class="flex gap-4 text-xs font-semibold tracking-widest text-slate-500 uppercase">
                {#if $users && $users.length > 0}
                    {@render pageButton('Users')}
                {/if}
                {@render pageButton('Login')}
                {@render pageButton('Register')}
            </nav>
        </div>
    </div>
</BaseModal>
