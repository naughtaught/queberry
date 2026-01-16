<script lang="ts">
    import '../app.css'

    import { Header } from '$lib'

    let { children } = $props()
</script>

<div class="fixed top-0 left-0 z-50 w-full">
    <Header />
</div>

{@render children()}
