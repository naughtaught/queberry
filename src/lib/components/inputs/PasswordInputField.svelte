<script lang="ts">
    import VisiblePasswordIcon from 'virtual:icons/material-symbols/lock-open-right-outline'
    import PasswordIcon from 'virtual:icons/material-symbols/lock-outline'

    import { onDestroy } from 'svelte'
    import { MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH, passwordVisibility } from '$lib/stores/app'

    let { password = $bindable(), name, touched = $bindable(), required = true } = $props()

    const togglePasswordVisibility = (event: { preventDefault: () => void }): void => {
        event.preventDefault()
        $passwordVisibility = !$passwordVisibility
    }

    onDestroy(() => {
        $passwordVisibility = false
    })
</script>

<label class="flex flex-col gap-1.5 text-xs font-semibold tracking-wider text-slate-400 uppercase"
    >{name}
    <div class="group relative">
        {#if !$passwordVisibility}
            <button
                type="button"
                class="absolute top-1/2 left-3 flex -translate-y-1/2 items-center focus:outline-none"
                onclick={togglePasswordVisibility}>
                <PasswordIcon class="h-5 w-5 text-slate-500 group-focus-within:text-primaryColor" />
            </button>
        {:else}
            <button
                type="button"
                class="absolute top-1/2 left-3 flex -translate-y-1/2 items-center focus:outline-none"
                onclick={togglePasswordVisibility}>
                <VisiblePasswordIcon class="h-5 w-5 text-slate-500 group-focus-within:text-primaryColor" />
            </button>
        {/if}
        <input
            {required}
            onblur={() => (touched = true)}
            minlength={MIN_PASSWORD_LENGTH}
            maxlength={MAX_PASSWORD_LENGTH}
            bind:value={password}
            class="w-full rounded-lg border border-slate-700 bg-slate-800/50 py-3 pr-4 pl-10 text-slate-100 transition-all placeholder:text-slate-500"
            placeholder="••••••"
            type={$passwordVisibility ? 'text' : 'password'} />
    </div>
</label>
