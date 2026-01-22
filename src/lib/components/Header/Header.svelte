<script lang="ts">
    import {
        appState,
        closeApp,
        HeaderButton,
        HeaderLink,
        minimizeApp,
        toggleFullscreen,
    } from '$lib'
</script>

<header
    class="-z-10 flex w-full max-w-screen items-center justify-between bg-transparent p-2"
    data-tauri-drag-region="true">
    <div class="z-50 flex min-w-34 shrink-0 items-center" data-tauri-drag-region="true">
        <img class="h-12 w-12" src="src/static/logo.svg" alt="" />
    </div>
    <nav class="mx-4 flex w-full flex-1 justify-center" data-tauri-drag-region="true">
        <ul
            data-tauri-drag-region="true"
            class="flex w-full max-w-2xl justify-center gap-x-4 font-semibold lg:gap-x-6 [&>li]:relative [&>li]:z-0 [&>li]:flex">
            <li>
                <a href="/">
                    <HeaderLink
                        name="Home"
                        path="M192-144v-456l288-216 288 216v456H552v-264H408v264H192Z" />
                </a>
            </li>
            <li>
                <a href="/video">
                    <HeaderLink
                        name="Video"
                        path="m276-276 289-119 119-289-289 119-119 289Zm204-156q-20 1-34-13.5T432-480q0-20 13.8-34t34.2-14q20.4 0 34.2 13.8Q528-500.4 528-480q0 20.4-13.8 34.2Q500.4-432 480-432Zm.276 336Q401-96 331-126q-70-30-122.5-82.5T126-330.958q-30-69.959-30-149.5Q96-560 126-629.5t82.5-122Q261-804 330.958-834q69.959-30 149.5-30Q560-864 629.5-834t122 82.5Q804-699 834-629.276q30 69.725 30 149Q864-401 834-331q-30 70-82.5 122.5T629.276-126q-69.725 30-149 30Z" />
                </a>
            </li>
        </ul>
    </nav>
    <nav class="flex shrink-0 justify-end">
        <ul class="flex space-x-4 [&>li]:relative [&>li]:flex">
            <li class="z-0">
                <button onclick={toggleFullscreen}>
                    {#if $appState.isFullscreen}
                        <HeaderButton
                            name="fullscreenOff"
                            path="M240-120v-120H120v-80h200v200h-80Zm400 0v-200h200v80H720v120h-80ZM120-640v-80h120v-120h80v200H120Zm520 0v-200h80v120h120v80H640Z" />
                    {:else}
                        <HeaderButton
                            name="fullscreenOn"
                            path="M144-144v-192h72v120h120v72H144Zm480 0v-72h120v-120h72v192H624ZM144-624v-192h192v72H216v120h-72Zm600 0v-120H624v-72h192v192h-72Z" />
                    {/if}
                </button>
            </li>
            <li>
                <button onclick={minimizeApp} data-action="minimize">
                    <HeaderButton name="minimizeApp" path="M240-120v-80h480v80H240Z" />
                </button>
            </li>
            <li>
                <button class="z-50" onclick={closeApp} data-action="close">
                    <HeaderButton
                        name="closeApp"
                        path="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z" />
                </button>
            </li>
        </ul>
    </nav>
</header>
