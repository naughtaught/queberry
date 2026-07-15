<script>
    import { resolve } from '$app/paths'

    const { credit } = $props()

</script>

<div class="flex items-center gap-3">
    <a href={resolve(`/person/?id=${credit.id}`, {})}>
        <div class="size-18 overflow-hidden rounded-lg border border-white/5 bg-white/5">
            <img
                alt="Headshot of {credit.name}"
                class="h-full w-full object-cover"
                src={credit.image
                    ? `https://image.tmdb.org/t/p/w185/${credit.image}`
                    : '/images/person-placeholder.png'} />
        </div>
        <div>
            <p class=" font-bold text-textColor">{credit.name}</p>
            <p class="text-sm text-slate-500">{credit.character}</p>
        </div>
    </a>
</div>
