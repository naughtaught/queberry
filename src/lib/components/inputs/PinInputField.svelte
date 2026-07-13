<script lang="ts">
    import { MAX_PIN_LENGTH, MIN_PIN_LENGTH } from '$lib/stores/app'
    import VisiblePasswordIcon from 'virtual:icons/material-symbols/lock-open-right-outline'
    import PasswordIcon from 'virtual:icons/material-symbols/lock-outline'

    let {
        pin = $bindable(),
        label = true,
        height = '',
        isUserSelect = false,
        func = () => {},
        labelText = 'Quick Access Pin',
        required = true,
    } = $props()
    let pinVisibility = $state(false)

    const togglePinVisibility = (event: { preventDefault: () => void }): void => {
        event.preventDefault()
        pinVisibility = !pinVisibility
    }

    const handleKeydown = (event: KeyboardEvent): void => {
        if (!isUserSelect) return
        if (event.code === 'Enter') func()
    }
</script>

<label class="flex flex-col gap-1.5 text-xs font-semibold tracking-wider text-slate-400 uppercase"
    >{#if label}{labelText}{/if}
    <div class="group relative">
        {#if !pinVisibility}
            <button
                type="button"
                class="absolute top-1/2 left-3 flex -translate-y-1/2 items-center focus:outline-none"
                onclick={togglePinVisibility}>
                <PasswordIcon class="h-5 w-5 text-slate-500 group-focus-within:text-primaryColor" />
            </button>
        {:else}
            <button
                type="button"
                class="absolute top-1/2 left-3 flex -translate-y-1/2 items-center focus:outline-none"
                onclick={togglePinVisibility}>
                <VisiblePasswordIcon class="h-5 w-5 text-slate-500 group-focus-within:text-primaryColor" />
            </button>
        {/if}
        <input
            onkeydown={(event) => handleKeydown(event)}
            {required}
            minlength={MIN_PIN_LENGTH}
            maxlength={MAX_PIN_LENGTH}
            bind:value={pin}
            class="w-full rounded-lg border border-slate-700 bg-slate-800/50 py-3 pr-4 pl-10 text-slate-100 transition-all placeholder:text-slate-500 {height}"
            placeholder="••••"
            type={pinVisibility ? 'text' : 'password'} />
    </div>
</label>
