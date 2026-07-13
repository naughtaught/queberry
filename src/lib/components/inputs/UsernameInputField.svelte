<script>
    import PersonIcon from 'virtual:icons/material-symbols/person'

    let { username = $bindable(), required = true } = $props()
</script>

<label class="flex flex-col gap-1.5 text-xs font-semibold tracking-wider text-slate-400 uppercase"
    >Username
    <div class="group relative">
        <span class="absolute top-1/2 left-3 -translate-y-1/2 text-slate-500 transition-colors">
            <PersonIcon class="h-5 w-5 group-focus-within:text-primaryColor" /></span>
        <input
            {required}
            class="w-full rounded-lg border border-slate-700 bg-slate-800/50 py-3 pr-4 pl-10 text-slate-100 transition-all outline-none placeholder:text-slate-500 focus:border-primaryColor focus:ring-2 focus:ring-primaryColor/50"
            placeholder="Enter username"
            bind:value={username}
            type="text" />
    </div>
</label>
