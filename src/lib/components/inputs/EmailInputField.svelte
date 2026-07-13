<script lang="ts">
    import EmailIcon from 'virtual:icons/material-symbols/alternate-email'

    let { email = $bindable(), required = true } = $props()
</script>

<label class="flex flex-col gap-1.5 text-xs font-semibold tracking-wider text-slate-400 uppercase">
    Email Address
    <div class="group relative">
        <span class="absolute top-1/2 left-3 -translate-y-1/2 text-slate-500 transition-colors">
            <EmailIcon class="h-5 w-5 group-focus-within:text-primaryColor" />
        </span>
        <input
            {required}
            class="peer w-full rounded-lg border border-slate-700 bg-slate-800/50 py-3 pr-4 pl-10 text-slate-100 transition-all not-only:placeholder:text-slate-500"
            placeholder="email@example.com"
            bind:value={email}
            type="email" />
    </div>
</label>
