<script lang="ts">
    let { label, checked = $bindable(), disabled = false, func = null } = $props()
</script>

<label
    class="group flex items-center gap-1.5 text-xs font-semibold tracking-wider uppercase
        {disabled ? 'cursor-default text-neutral-500' : 'cursor-pointer text-slate-300'}">
    <input {disabled} onchange={func} type="checkbox" class="hidden" bind:checked />
    <span
        class="flex h-4 w-4 items-center justify-center rounded border-2 {disabled
            ? 'cursor-default border-neutral-500'
            : checked
              ? 'border-primaryColor bg-primaryColor'
              : 'border-slate-300'}">
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
