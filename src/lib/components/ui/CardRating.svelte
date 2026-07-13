<script lang="ts">
    const { rating } = $props()

    const ratingLevel = $derived(rating === 100 ? 90 : Math.floor(rating / 10) * 10)
    const colorVar = $derived(`var(--color-rating-${ratingLevel})`)

</script>

<svg viewBox="-12 -10 24 20" xmlns="http://www.w3.org/2000/svg">
    <text
        x="0"
        y="0"
        dominant-baseline="central"
        text-anchor="middle"
        font-size="10"
        paint-order="stroke"
        stroke="#000000"
        stroke-width="1.2px"
        fill={colorVar}
        font-weight="700">
        {rating}
    </text>
</svg>
