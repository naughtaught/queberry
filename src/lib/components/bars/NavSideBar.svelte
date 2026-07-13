<script lang="ts">
    import { page } from '$app/state'
    import SearchIcon from 'virtual:icons/material-symbols/search'
    import HomeIcon from 'virtual:icons/ic/baseline-home'
    import DiscoverIcon from 'virtual:icons/weui/discover-filled'
    import SettingsIcon from 'virtual:icons/material-symbols/settings'
    import FavouriteIcon from 'virtual:icons/material-symbols/favorite'
    import WatchlistIcon from 'virtual:icons/material-symbols/bookmark-sharp'
    import DownloadIcon from 'virtual:icons/material-symbols/download'
    import TransferIcon from 'virtual:icons/bx/transfer'
    // import ListsIcon from 'virtual:icons/material-symbols/patient-list'
    // import PluginsIcon from 'virtual:icons/clarity/plugin-solid'
    import AddIcon from 'virtual:icons/material-symbols/add-box-outline'

    import { goto } from '$app/navigation'
    import { user } from '$lib/stores/user'
    import { getAvatar } from '$lib/functions/utility/getAvatar'
    import NavLink from '$lib/components/links/NavLink.svelte'
    import { modals } from '$lib/stores/app'

    let avatarSrc = $state('')

    $effect(() => {
        if ($user?.avatar) getAvatar($user.avatar).then((src) => (avatarSrc = src))
    })
</script>

<aside class="sidebar-gradient flex h-fit min-h-screen w-40 shrink-0 flex-col border-r border-textColor/5">
    <div class="flex items-center gap-2 p-4">
        <img class="h-8 w-8" src="/images/logo.png" alt="" />
        <h1 class="text-lg leading-tight font-bold tracking-widest">queberry</h1>
    </div>
    <nav class="mt-2 flex-1 px-1">
        <div class="mb-2 px-4 text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">Media</div>
        <NavLink text="Home" href="/">
            <HomeIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
        <NavLink text="Discover" href="/discover">
            <DiscoverIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/discover'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
        <NavLink text="Watchlist" href="/watchlist">
            <WatchlistIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/watchlist'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
        <NavLink text="Favourites" href="/favourites">
            <FavouriteIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/favourites'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
        <!-- <NavLink text="Lists" href="/lists">
            <ListsIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/lists'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink> -->
        <button
            onclick={() => ($modals.search = true)}
            class="group flex items-center gap-2 rounded-xl px-4 py-3 text-textColor transition-all">
            <SearchIcon
                class="text-xl group-hover:text-primaryColor {$modals.search
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
            <span
                class="relative flex cursor-pointer items-center pr-1 text-sm font-semibold tracking-wide after:absolute after:-bottom-[1.5px] after:left-0 after:h-0.5 after:bg-primaryColor after:transition-all after:duration-100 group-hover:after:w-full"
                class:after:w-full={$modals.search}>
                Search
            </span>
        </button>
    </nav>
    <nav>
        <div class="mb-2 px-4 text-[10px] font-bold tracking-[0.2em] text-slate-500 uppercase">General</div>
        <NavLink text="Add Media" href="/add">
            <AddIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/add'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
        <NavLink text="Transfers" href="/transfers">
            <TransferIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/transfers'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
        <NavLink text="Downloads" href="/downloads">
            <DownloadIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/downloads'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
        <!-- <NavLink text="Plugins" href="/pluginList">
            <PluginsIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/pluginLists'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink> -->
        <NavLink text="Settings" href="/settings">
            <SettingsIcon
                class="text-xl group-hover:text-primaryColor {page.url.pathname === '/settings'
                    ? 'text-primaryColor'
                    : 'text-textColor'}" />
        </NavLink>
    </nav>
    <div class="mt-5 flex w-full flex-col items-center rounded-2xl px-2 pb-5">
        <button
            onclick={() => {
                goto('/')
                $modals.user = true
            }}
            class="group relative"
            aria-label="Switch User">
            <img
                class="size-25 rounded-xl border-3 border-textColor/10 bg-cover bg-center group-hover:border-primaryColor"
                src={avatarSrc}
                alt="User Avatar" />
        </button>
    </div>
</aside>
