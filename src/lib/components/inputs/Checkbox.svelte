<script lang="ts">
    let { label = '', checked = $bindable(), disabled = false, func = null, id = undefined } = $props()

    const checkboxId = $derived(id || `checkbox-${Math.random().toString(36).slice(2, 9)}`)

    const getBorderClass = (): string => {
        if (disabled) return 'cursor-default border-neutral-500'
        if (checked) return 'border-primaryColor bg-primaryColor'
        return 'border-slate-300'
    }

    const handleChange = (e: Event): void => {
        checked = (e.target as HTMLInputElement).checked
        func?.(e)
    }
</script>

<input {disabled} onchange={handleChange} type="checkbox" class="sr-only" {checked} id={checkboxId} />
<label
    for={checkboxId}
    class="group flex items-center gap-1.5 text-xs font-semibold tracking-wider uppercase
    {disabled ? 'cursor-default text-neutral-500' : 'cursor-pointer text-slate-300'}">
    <span class="flex h-4 w-4 items-center justify-center rounded border-2 {getBorderClass()}">
        <svg
            class="h-3 w-3 text-black {checked ? 'block' : 'hidden'}"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="4">
            <path d="M20 6L9 17L4 12" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
    </span>
    {label}
</label>
