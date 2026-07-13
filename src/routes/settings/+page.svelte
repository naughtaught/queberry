<script lang="ts">
    import UpdateUserForm from '$lib/components/forms/UpdateUserForm.svelte'
    import Checkbox from '$lib/components/inputs/Checkbox.svelte'
    import DraggableOrder from '$lib/components/inputs/DraggableOrder.svelte'
    import PluginKeys from '$lib/components/inputs/PluginKeys.svelte'
    import Select from '$lib/components/inputs/Select.svelte'
    import Slider from '$lib/components/inputs/Slider.svelte'
    import ToggleCheckbox from '$lib/components/inputs/ToggleCheckbox.svelte'
    import ToggleSwitch from '$lib/components/inputs/ToggleSwitch.svelte'
    import ConfirmationModal from '$lib/components/modals/ConfirmationModal.svelte'
    import Patchnotes from '$lib/components/modals/Patchnotes.svelte'

    import LocalMediaManager from '$lib/components/modals/LocalMediaManager.svelte'
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
    import { languages } from '$lib/data/languages'
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setEnabledPlugins } from '$lib/functions/plugins/setEnabledPlugins'
    import { updateGlobalSettings } from '$lib/functions/user/updateGlobalSettings'
    import { getAvatar } from '$lib/functions/utility/getAvatar'
    import { updateCarousels } from '$lib/functions/utility/updateCarousels'
    import {
        appData,
        AUDIO_OPTIONS,
        directories,
        loadingStates,
        MOVIE_RATINGS,
        parentalControlsAreEnabled,
        primaryUser,
        PRIORITY_LANGAUGES,
        RESOLUTION_ORDER,
        SCREENSAVER_TIMEOUTS,
        TV_RATINGS,
        VIDEO_OPTIONS,
    } from '$lib/stores/app'
    import {
        enabledResolverPlugins,
        installedIndexerPlugins,
        installedResolverPlugins,
        installedUtilityPlugins,
    } from '$lib/stores/plugins'
    import { hashBlacklist, restrictedContent, settings, user, users } from '$lib/stores/user'
    import { AUDIO_CHANNELS } from '$lib/stores/video'
    import type { Sql } from '$lib/types/sql'
    import { open } from '@tauri-apps/plugin-dialog'

    let avatarSrc = $state('')
    let parentalControlUser = $derived($primaryUser)
    let restrictedMovies: string[] = $derived($restrictedContent.movies)
    let restrictedTv: string[] = $derived($restrictedContent.tv)
    let isUpdateModalOpen = $state(false)
    let isMediaManagerOpen = $state(false)
    let isPatchNotesModalOpen = $state(false)
    let confirmationModalConfig = $state({
        isOpen: false,
        title: '',
        message: '',
        action: () => {},
    })

    $effect(() => {
        if ($user?.avatar) getAvatar($user.avatar).then((src) => (avatarSrc = src))
    })

    const languageOptions = [
        { name: 'Source', value: 'Source' },
        ...Array.from(
            new Set(
                languages.map((x) => ({
                    name: x.name,
                    value: x.code,
                })),
            ),
        ).sort((a, b) => {
            const aName = a.name ?? ''
            const bName = b.name ?? ''

            const aPriority = PRIORITY_LANGAUGES.indexOf(aName)
            const bPriority = PRIORITY_LANGAUGES.indexOf(bName)
            if (aPriority !== -1 && bPriority !== -1) {
                return aPriority - bPriority
            }
            if (aPriority !== -1) return -1
            if (bPriority !== -1) return 1
            return aName.localeCompare(bName)
        }),
    ]

    const subtitleOptions = Array.from(
        new Set(
            languages.map((x) => ({
                name: x.name,
                value: x.code,
            })),
        ),
    ).sort((a, b) => {
        const aName = a.name ?? ''
        const bName = b.name ?? ''

        const aPriority = PRIORITY_LANGAUGES.indexOf(aName)
        const bPriority = PRIORITY_LANGAUGES.indexOf(bName)
        if (aPriority !== -1 && bPriority !== -1) {
            return aPriority - bPriority
        }
        if (aPriority !== -1) return -1
        if (bPriority !== -1) return 1
        return aName.localeCompare(bName)
    })

    const updateSettings = async (setting: string, value: unknown): Promise<void> => {
        try {
            const response = await invokeFunction('update_user_settings', {
                settings: {
                    ...$settings,
                    [setting]: value,
                },
            })
            if (!response.success) throw response.error

            $settings = {
                ...$settings,
                [setting]: value,
            }
        } catch (error) {
            handleError(error)
        }
    }

    const updateSettingsArray = async (
        setting: keyof Pick<
            Sql.UserSettings,
            | 'disabledPlugins'
            | 'excludedResolutions'
            | 'excludedVideoFormats'
            | 'excludedAudioOptions'
            | 'excludedSources'
        >,
        value: string | number,
    ): Promise<void> => {
        if (!$user) return
        const currentArray = $settings[setting]
        const index = currentArray.findIndex((x) => x === value)

        const newArray =
            index === -1
                ? [...currentArray, value]
                : [...currentArray.slice(0, index), ...currentArray.slice(index + 1)]

        await updateSettings(setting, newArray)

        if (setting === 'disabledPlugins') {
            const pluginsResp = await setEnabledPlugins()
            if (!pluginsResp.success) throw pluginsResp.error
        }
    }

    const getContentRatings = async (): Promise<void> => {
        try {
            const content_ratings = await invokeFunction('get_user_content_ratings', {
                userId: parentalControlUser,
            })
            if (!content_ratings.success) throw content_ratings.error

            if (content_ratings.data) {
                restrictedTv = content_ratings.data.restrictedTv || []
                restrictedMovies = content_ratings.data.restrictedMovies || []
            }
        } catch (error) {
            handleError(error)
        }
    }

    const updateContentRatings = async (option: string, type: 'movies' | 'tv'): Promise<void> => {
        try {
            if (type === 'movies') {
                restrictedMovies = restrictedMovies.includes(option)
                    ? restrictedMovies.filter((x) => x !== option)
                    : [...restrictedMovies, option]
            } else {
                restrictedTv = restrictedTv.includes(option)
                    ? restrictedTv.filter((x) => x !== option)
                    : [...restrictedTv, option]
            }

            const content_ratings = await invokeFunction('upsert_user_content_ratings', {
                userId: parentalControlUser,
                restrictedMovies,
                restrictedTv,
            })

            if (!content_ratings.success) throw content_ratings.error

            if (content_ratings.data) {
                restrictedTv = content_ratings.data.restrictedTv || []
                restrictedMovies = content_ratings.data.restrictedMovies || []

                if ($user?.id === parentalControlUser) {
                    $restrictedContent.movies = restrictedMovies
                    $restrictedContent.tv = restrictedTv
                }
            }
        } catch (error) {
            handleError(error)
        }
    }

    const setConfirmationConfig = (title: string, action: () => void): void => {
        confirmationModalConfig = {
            isOpen: true,
            title: `${title}`,
            message: `Are you sure you want to reset your ${title}?`,
            action,
        }
        switch (title) {
            case 'Delete User':
                confirmationModalConfig.message = `Are you sure you want to remove this user? 
                This will only remove the user from this device.`
                break
            case 'Delete Account':
                confirmationModalConfig.message = `Are you sure you want to delete your account?
                This is immediate and unrecoverable.`
                break
            case 'Reset Media':
                confirmationModalConfig.message = `Are you sure you want to reset your media?
                All media states and ratings will be reset.`
                break
            default:
                confirmationModalConfig.message = confirmationModalConfig.message
                break
        }
    }

    const deleteUser = async (): Promise<void> => {
        if (!$user) return
        try {
            const response = await invokeFunction('delete_user', { userId: $user.id })
            if (!response.success) throw response.error

            $user = null

            const getUsers = await invokeFunction('get_all_users', {})

            if (!getUsers.success) throw getUsers.error

            $users = getUsers.data
        } catch (error) {
            handleError(error)
        }
    }

    const resetMedia = async (): Promise<void> => {
        if (!$user) return
        try {
            const response = await invokeFunction('api_reset_user_data', {
                postgresId: $user.postgresId,
                token: $user.token,
            })
            if (!response.success) throw response.error

            const blacklisted = await invokeFunction('delete_users_blacklisted', {
                userId: $user.id,
            })
            if (!blacklisted.success) throw blacklisted.error
            $hashBlacklist = []

            const resp = await updateCarousels()
            if (!resp.success) throw resp.error
        } catch (error) {
            handleError(error)
        }
    }

    const deleteAccount = async (): Promise<void> => {
        if (!$user) return
        try {
            const response = await invokeFunction('api_delete_user', {
                postgresId: $user.postgresId,
                token: $user.token,
            })
            if (!response.success) throw response.error

            deleteUser()
        } catch (error) {
            handleError(error)
        }
    }

    const resetApiKey = async (): Promise<void> => {
        if (!$user) return
        try {
            const response = await invokeFunction('api_reset_token', {
                postgresId: $user.postgresId,
                token: $user.token,
            })
            if (!response.success) throw response.error

            const sqlUpdate = await invokeFunction('update_user', {
                updates: {
                    userId: $user.id,
                    token: response.data.token,
                },
            })
            if (!sqlUpdate.success) throw sqlUpdate.error

            $user = sqlUpdate.data

            const allUsers = await invokeFunction('get_all_users', {})
            if (!allUsers.success) throw allUsers.error

            $users = allUsers.data
        } catch (error) {
            handleError(error)
        }
    }

    const setMediaDirectory = async (type: string): Promise<void> => {
        const currentDir = $directories[type === 'tv' ? 'tv' : 'movies']

        const folder = await open({
            multiple: false,
            directory: true,
        })

        if (folder) {
            try {
                $loadingStates.isLocalMediaLoading = true

                await setMediaFolder(type, folder)

                if (currentDir) await deleteLocalMediaFiles(currentDir)
                scanFolder(folder)
            } catch (error) {
                handleError(error)
            } finally {
                $loadingStates.isLocalMediaLoading = false
            }
        }
    }

    const resetMediaDirectory = async (type: string): Promise<void> => {
        const currentDir = $directories[type === 'tv' ? 'tv' : 'movies']

        try {
            $loadingStates.isLocalMediaLoading = true
            await setMediaFolder(type)

            if (currentDir) deleteLocalMediaFiles(currentDir)
        } catch (error) {
            handleError(error)
        } finally {
            $loadingStates.isLocalMediaLoading = false
        }
    }

    const setMediaFolder = async (type: string, file: string | null = null): Promise<void> => {
        $directories[type === 'tv' ? 'tv' : 'movies'] = file
        await updateGlobalSettings($user!, $parentalControlsAreEnabled)
    }

    const deleteLocalMediaFiles = async (directory: string): Promise<void> => {
        const response = await invokeFunction('delete_local_media_by_directory', {
            directory,
        })
        if (!response.success) throw response.error
    }

    const scanFolder = async (directory: string): Promise<void> => {
        const response = await invokeFunction('scan_local_folder', {
            directory,
        })
        if (!response.success) throw response.error
    }

    // TODO keyboard mapping
</script>

{#if isUpdateModalOpen}
    <UpdateUserForm bind:isUpdateModalOpen />
{/if}

{#if isPatchNotesModalOpen}
    <Patchnotes bind:isPatchNotesModalOpen />
{/if}

{#if isMediaManagerOpen}
    <LocalMediaManager bind:isMediaManagerOpen />
{/if}

{#if confirmationModalConfig.isOpen}
    <ConfirmationModal bind:config={confirmationModalConfig} />
{/if}

<section class="mx-auto flex h-[calc(100vh-88px)] w-full justify-center overflow-y-auto py-12">
    <div class="grid grid-cols-12 gap-6">
        <!-- Account Settings -->
        <section class="glass-panel sidebar-gradient col-span-8 rounded-lg p-8 shadow-2xl">
            <div class="mb-8 flex items-center justify-between">
                <div>
                    <h3 class="mt-1 text-xl font-bold">Account Settings</h3>
                </div>
                <div>
                    <button
                        onclick={() => {
                            isUpdateModalOpen = true
                        }}
                        class="rounded-full border border-white/5 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                        >Edit</button>
                </div>
            </div>
            <div class="flex flex-row items-center gap-8">
                <div class="group relative">
                    <div class="h-32 w-32 overflow-hidden rounded-lg border-4 border-black">
                        <img alt="Profile Avatar" class="h-full w-full object-cover" src={avatarSrc} />
                    </div>
                </div>
                <div class="grid w-full flex-1 grid-cols-2 gap-6">
                    <div>
                        <h4>Username</h4>
                        <p class="text-xs font-medium">{$user?.username}</p>
                    </div>
                    <div>
                        <h4>Email Address</h4>
                        <p class="text-xs font-medium">{$user?.email}</p>
                    </div>
                    <div class="my-auto items-center">
                        <button
                            onclick={() => {
                                setConfirmationConfig('Delete Account', () => deleteAccount())
                            }}
                            class="rounded-lg border border-white/5 bg-slate-800 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                            >Delete Account</button>
                    </div>
                    <div class="my-auto items-center">
                        <button
                            onclick={() => {
                                setConfirmationConfig('Delete User', () => deleteUser())
                            }}
                            class="rounded-lg border border-white/5 bg-slate-800 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                            >Delete User</button>
                    </div>
                    <div class="my-auto items-center">
                        <button
                            onclick={() => {
                                setConfirmationConfig('Reset Media', () => resetMedia())
                            }}
                            class="rounded-lg border border-white/5 bg-slate-800 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                            >Reset Media</button>
                    </div>
                    <div>
                        <button
                            onclick={resetApiKey}
                            class="rounded-lg border border-white/5 bg-slate-800 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                            >Reset API Key</button>
                    </div>
                </div>
            </div>
        </section>
        <!-- App Info -->
        <section class="glass-panel sidebar-gradient col-span-4 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">App Info</h3>
            </div>
            <div class="space-y-8">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Version</p>
                        <p class="text-xs">Your Current Version</p>
                    </div>
                    <p>{$appData.currentVersion}</p>
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Patchnotes</p>
                        <p class="text-xs">Latest Changes</p>
                    </div>
                    <button
                        onclick={() => {
                            isPatchNotesModalOpen = true
                        }}
                        class="rounded-lg border border-white/5 bg-slate-800 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                        >Open</button>
                </div>
            </div>
        </section>
        <!-- Playback & Audio -->
        <section class="glass-panel sidebar-gradient col-span-6 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">Playback & Audio</h3>
            </div>
            <div class="space-y-8">
                <div class="flex items-center justify-between space-x-5">
                    <div>
                        <p class="text-sm font-bold">Default Audio Configuration</p>
                        <p class="text-xs">Language for audio tracks</p>
                    </div>
                    <div class="w-64">
                        <Select
                            maxWidth="w-full"
                            options={$AUDIO_CHANNELS}
                            bind:activeOption={$settings.audioChannel}
                            func={() => updateSettings('audioChannels', $settings.audioChannel)}
                            name="" />
                    </div>
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Preferred Audio</p>
                        <p class="text-xs">Language for audio tracks</p>
                    </div>
                    <div class="w-64">
                        <Select
                            maxWidth="w-full"
                            options={languageOptions}
                            bind:activeOption={$settings.preferredAudioLanguage}
                            func={() => updateSettings('preferredAudioLanguage', $settings.preferredAudioLanguage)}
                            name="" />
                    </div>
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Continuous Playback</p>
                        <p class="text-xs">Automatically play the next episode in a series</p>
                    </div>
                    <ToggleCheckbox
                        bind:value={$settings.autoplay}
                        func={() => updateSettings('autoplay', $settings.autoplay)} />
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Season Complete</p>
                        <p class="text-xs">Only continue playback if next season is completed</p>
                    </div>

                    <ToggleCheckbox
                        bind:value={$settings.seasonCompletionRequired}
                        func={() => updateSettings('seasonCompletionRequired', $settings.seasonCompletionRequired)} />
                </div>
                <div>
                    <Slider
                        min={0}
                        max={100}
                        bind:value={$settings.volume}
                        label="Default Volume"
                        func={() => {
                            updateSettings('volume', $settings.volume)
                        }} />
                </div>
            </div>
        </section>
        <!-- Display -->
        <section class="glass-panel sidebar-gradient col-span-6 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">Display</h3>
            </div>
            <div class="space-y-8">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Screensaver</p>
                        <p class="text-xs">Idle time before activation</p>
                    </div>
                    <div class="w-64">
                        <Select
                            maxWidth="w-full"
                            options={SCREENSAVER_TIMEOUTS}
                            bind:activeOption={$settings.screensaverTimeout}
                            func={() => updateSettings('screensaverTimeout', $settings.screensaverTimeout)}
                            name="" />
                    </div>
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Video Time Display</p>
                    </div>
                    <ToggleSwitch
                        options={['Duration', 'Time Remaining']}
                        bind:value={$settings.durationDisplay}
                        func={() => {
                            updateSettings('durationDisplay', $settings.durationDisplay)
                        }} />
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">User Ratings</p>
                        <p class="text-xs">Enable ratings</p>
                    </div>

                    <ToggleCheckbox
                        bind:value={$settings.enableUserRatings}
                        func={() => updateSettings('enableUserRatings', $settings.enableUserRatings)} />
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Fullscreen</p>
                        <p class="text-xs">Open the app in fullscreen</p>
                    </div>
                    <ToggleCheckbox
                        bind:value={$settings.openAppFullscreen}
                        func={() => updateSettings('openAppFullscreen', $settings.openAppFullscreen)} />
                </div>
                <div>
                    <Slider
                        zeroPoint={true}
                        min={50}
                        max={150}
                        step={1}
                        bind:value={$settings.imageScaling}
                        label="Image Scale"
                        func={() => {
                            updateSettings('imageScaling', $settings.imageScaling)
                        }} />
                </div>
            </div>
        </section>
        <!-- Subtitles -->
        <section class="glass-panel sidebar-gradient col-span-6 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">Subtitles</h3>
            </div>
            <div class="space-y-8">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Preferred Language</p>
                        <p class="text-xs">Automatically select this subtitle track</p>
                    </div>
                    <Select
                        options={subtitleOptions}
                        bind:activeOption={$settings.preferredSubtitleLanguage}
                        func={() => updateSettings('preferredSubtitleLanguage', $settings.preferredSubtitleLanguage)}
                        name="" />
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Behaviour</p>
                        <p class="text-xs">
                            {#if $settings.subtitleDisplay === 'Auto'}
                                Automatically determine
                            {:else if $settings.subtitleDisplay === 'On'}
                                Subtitles are always on
                            {:else}
                                Subtitles are always off
                            {/if}
                        </p>
                    </div>
                    <ToggleSwitch
                        options={['On', 'Auto', 'Off']}
                        bind:value={$settings.subtitleDisplay}
                        func={() => {
                            updateSettings('subtitleDisplay', $settings.subtitleDisplay)
                        }} />
                </div>
            </div>
        </section>
        <!-- General -->
        <section class="glass-panel sidebar-gradient col-span-6 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">General</h3>
            </div>
            <div class="space-y-8">
                <div>
                    <Slider
                        min={70}
                        max={95}
                        bind:value={$settings.completionPercent}
                        label="Mark video as complete after"
                        func={() => {
                            updateSettings('completionPercent', $settings.completionPercent)
                        }} />
                </div>
            </div>
        </section>
        <!-- Timings -->
        <section class="glass-panel sidebar-gradient col-span-12 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">Timings</h3>
            </div>
            <div class="grid grid-cols-2 gap-x-8 gap-y-5">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Intros</p>
                        <p class="text-xs">Skip Intros</p>
                    </div>

                    <ToggleCheckbox
                        bind:value={$settings.skipIntro}
                        func={() => updateSettings('skipIntro', $settings.skipIntro)} />
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Recaps</p>
                        <p class="text-xs">Skip Recaps</p>
                    </div>

                    <ToggleCheckbox
                        bind:value={$settings.skipRecap}
                        func={() => updateSettings('skipRecap', $settings.skipRecap)} />
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Credits</p>
                        <p class="text-xs">Skip Credits</p>
                    </div>

                    <ToggleCheckbox
                        bind:value={$settings.skipCredits}
                        func={() => updateSettings('skipCredits', $settings.skipCredits)} />
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Previews</p>
                        <p class="text-xs">Skip Previews</p>
                    </div>

                    <ToggleCheckbox
                        bind:value={$settings.skipPreview}
                        func={() => updateSettings('skipPreview', $settings.skipPreview)} />
                </div>
            </div>
        </section>

        <!-- Local Media & Downloads -->
        <section class="glass-panel sidebar-gradient col-span-12 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">Local Media & Downloads</h3>
            </div>

            <div class="space-y-8">
                {#if $loadingStates.isLocalMediaLoading}
                    <div class="flex min-h-26 items-center">
                        <LoadingSpinner />
                    </div>
                {:else}
                    <div class="flex w-full items-center">
                        <div class="w-58">
                            <p class="text-sm font-bold">TV Directory</p>
                            <p class="text-xs">Set local media directory for tv shows</p>
                        </div>
                        <p class="flex-1 text-left text-xs">
                            {#if $directories.tv}{$directories.tv}{/if}
                        </p>
                        <div class="space-x-1">
                            <button
                                class="h-8.5 rounded-lg bg-slate-800 px-6 font-bold shadow-lg hover:text-primaryColor"
                                onclick={() => {
                                    setMediaDirectory('tv')
                                }}>Set</button>
                            {#if $directories.tv}
                                <button
                                    class="h-8.5 rounded-lg bg-slate-800 px-6 font-bold shadow-lg hover:text-red-500"
                                    onclick={() => {
                                        resetMediaDirectory('tv')
                                    }}>Reset</button
                                >{/if}
                        </div>
                    </div>
                    <div class="flex w-full items-center">
                        <div class="w-58">
                            <p class="text-sm font-bold">Movie Directory</p>
                            <p class="text-xs">Set local media directory for movies</p>
                        </div>
                        <p class="flex-1 text-left text-xs">
                            {#if $directories.movies}{$directories.movies}{/if}
                        </p>
                        <div class="space-x-1">
                            <button
                                class="h-8.5 rounded-lg bg-slate-800 px-6 font-bold shadow-lg hover:text-primaryColor"
                                onclick={() => {
                                    setMediaDirectory('movies')
                                }}>Set</button>
                            {#if $directories.movies}
                                <button
                                    class="h-8.5 rounded-lg bg-slate-800 px-6 font-bold shadow-lg hover:text-red-500"
                                    onclick={() => {
                                        resetMediaDirectory('movies')
                                    }}>Reset</button
                                >{/if}
                        </div>
                    </div>
                {/if}
                {#if $directories.movies || $directories.tv}
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm font-bold">Manage Local Media</p>
                            <p class="text-xs">Configure database entries</p>
                        </div>
                        <button
                            onclick={() => (isMediaManagerOpen = true)}
                            class="rounded-lg border border-white/5 bg-slate-800 px-6 py-2 text-xs font-bold transition-colors hover:bg-white/10"
                            >Open</button>
                    </div>
                {/if}
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Concurrent Downloads</p>
                        <p class="text-xs">Set max active downloads</p>
                    </div>
                    <div class="flex items-center">
                        <input
                            onchange={() => {
                                const value = Math.round($settings.maxConcurrentDownloads)
                                $settings.maxConcurrentDownloads = value
                                updateSettings('maxConcurrentDownloads', value)
                            }}
                            type="number"
                            placeholder="5"
                            bind:value={$settings.maxConcurrentDownloads}
                            min="1"
                            max="10"
                            step="1"
                            class="w-24 rounded border border-slate-700 bg-slate-800/50 px-2 text-right text-xs text-textColor"
                            aria-label="Source Size Limit" />
                    </div>
                </div>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-bold">Download Speed</p>
                        <p class="text-xs">Set max download speed for individual downloads in mbps</p>
                    </div>
                    <div class="flex items-center">
                        <input
                            onchange={() => {
                                const value = Math.round($settings.downloadRateLimit)
                                $settings.downloadRateLimit = value
                                updateSettings('downloadRateLimit', value)
                            }}
                            type="number"
                            placeholder="0 Mbps"
                            bind:value={$settings.downloadRateLimit}
                            min="0"
                            max=""
                            step="1"
                            class="w-24 rounded border border-slate-700 bg-slate-800/50 px-2 text-right text-xs text-textColor"
                            aria-label="Source Size Limit" />
                    </div>
                </div>
            </div>
        </section>
        <!-- Parental Controls -->
        <section class="glass-panel sidebar-gradient col-span-12 rounded-lg p-8 shadow-2xl">
            <div class="mb-8">
                <h3 class="mt-1 text-xl font-bold">Parental Controls</h3>
            </div>
            <div class="space-y-8">
                {#if $user?.id === $primaryUser}
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm font-bold">Enable</p>
                            <p class="text-xs">Toggle parental control content restrictions</p>
                        </div>
                        <ToggleCheckbox
                            bind:value={$parentalControlsAreEnabled}
                            func={() => updateGlobalSettings($user!, $parentalControlsAreEnabled)} />
                    </div>
                {/if}
                <div class="mt-4 text-sm font-semibold">
                    <div class="mb-5 flex border-b pb-1 text-textColor">
                        <div class="w-1/3 uppercase">
                            {#if $user?.id === $primaryUser}
                                User
                            {/if}
                        </div>
                        <div class="w-1/3 text-center uppercase">Restricted TV</div>
                        <div class="w-1/3 text-center uppercase">Restricted Movies</div>
                    </div>
                    <div class="flex text-textColor">
                        <div class="w-1/3">
                            {#if $user?.id === $primaryUser}
                                <Select
                                    maxWidth="w-full"
                                    options={$users.map((x) => ({
                                        name: x.username,
                                        value: x.id,
                                    }))}
                                    bind:activeOption={parentalControlUser}
                                    name=""
                                    func={() => {
                                        getContentRatings()
                                    }} />
                            {/if}
                        </div>
                        <div class="w-1/3 text-center">
                            <div class="mx-auto grid w-full justify-center gap-3 text-center">
                                {#each TV_RATINGS as option, index (index)}
                                    <Checkbox
                                        label={option}
                                        disabled={$user?.id !== $primaryUser}
                                        checked={restrictedTv.includes(option)}
                                        func={() => {
                                            updateContentRatings(option, 'tv')
                                        }} />
                                {/each}
                            </div>
                        </div>
                        <div class="w-1/3 text-center">
                            <div class="mx-auto grid w-full justify-center gap-3 text-center">
                                {#each MOVIE_RATINGS as option, index (index)}
                                    <Checkbox
                                        label={option}
                                        disabled={$user?.id !== $primaryUser}
                                        checked={restrictedMovies.includes(option)}
                                        func={() => {
                                            updateContentRatings(option, 'movies')
                                        }} />
                                {/each}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        <!-- Plugins -->
        <section class="glass-panel sidebar-gradient z-0 col-span-12 rounded-lg p-8 shadow-2xl">
            <div class="mb-10 flex flex-col justify-between gap-4 md:flex-row md:items-end">
                <div>
                    <h3 class="mt-1 text-xl font-bold">Plugin Settings</h3>
                    <p class="mt-2 max-w-2xl text-xs">Configure how the app uses installed plugins.</p>
                </div>
            </div>
            <div class="grid grid-cols-3 gap-x-8 gap-y-5">
                <div class="col-span-2 h-full space-y-6">
                    <div class="h-full rounded-lg border border-white/5 p-5">
                        <h4 class="mb-4">Apikeys</h4>
                        <div class="grid h-[90%] space-y-6">
                            <div class="flex flex-col">
                                <PluginKeys func={updateSettings} />
                            </div>
                        </div>
                    </div>
                </div>
                <div class="space-y-6">
                    <div class="rounded-lg border border-white/5 p-5">
                        <h4 class="mb-4">Source Size Limit</h4>
                        <div class="flex items-center">
                            <input
                                onchange={() => {
                                    const value = Math.round($settings.fileSizeLimit)
                                    $settings.fileSizeLimit = value
                                    updateSettings('fileSizeLimit', value)
                                }}
                                type="number"
                                placeholder="File Size GB"
                                bind:value={$settings.fileSizeLimit}
                                min="0"
                                max=""
                                step="1"
                                class="w-24 rounded border border-slate-700 bg-slate-800/50 px-2 text-xs text-textColor"
                                aria-label="Source Size Limit" />
                            <span class="ml-2 text-sm">GB</span>
                        </div>
                    </div>
                </div>
                <div class="h-full space-y-6">
                    <div class="h-full rounded-lg border border-white/5 p-5">
                        <h4 class="mb-4">Filtered Formats</h4>
                        <div class="grid h-[90%] grid-cols-2 gap-x-8">
                            {#each VIDEO_OPTIONS as option, index (index)}
                                <Checkbox
                                    label={option}
                                    checked={$settings.excludedResolutions.includes(option)}
                                    func={() => {
                                        updateSettingsArray('excludedResolutions', option)
                                    }} />
                            {/each}
                        </div>
                    </div>
                </div>
                <div class="h-full space-y-6">
                    <div class="h-full rounded-lg border border-white/5 p-5">
                        <h4 class="mb-4">Filtered Resolutions</h4>
                        <div class="grid h-[90%] space-y-6">
                            {#each RESOLUTION_ORDER as option, index (index)}
                                <Checkbox
                                    label={option}
                                    checked={$settings.excludedResolutions.includes(option)}
                                    func={() => {
                                        updateSettingsArray('excludedResolutions', option)
                                    }} />
                            {/each}
                        </div>
                    </div>
                </div>
                <div class="grid-cols-1 space-y-6">
                    <div class="rounded-lg border border-white/5 p-5">
                        <div class="mb-4 flex items-center justify-between">
                            <h4>Sort Order Priority</h4>
                        </div>
                        <DraggableOrder
                            onUpdate={() => {
                                updateSettings('indexerSortCriteria', $settings.indexerSortCriteria)
                            }} />
                    </div>
                </div>
                <div class="space-y-6">
                    <div class="rounded-lg border border-white/5 p-5">
                        <h4 class="mb-4">Filtered Audio</h4>
                        <div class="grid gap-3">
                            {#each AUDIO_OPTIONS as option, index (index)}
                                <Checkbox
                                    label={option}
                                    checked={$settings.excludedAudioOptions.includes(option)}
                                    func={() => {
                                        updateSettingsArray('excludedAudioOptions', option)
                                    }} />
                            {/each}
                        </div>
                    </div>
                </div>
                <div class="h-full space-y-6">
                    <div class="h-full rounded-lg border border-white/5 p-5">
                        <h4 class="mb-4">Preferred Resolver</h4>
                        <div class="grid h-[90%] space-y-6">
                            {#if $enabledResolverPlugins?.length > 0}
                                <div>
                                    <div class="grid grid-cols-2 gap-x-2 gap-y-2">
                                        {#each $enabledResolverPlugins as plugin (plugin.id)}
                                            <button
                                                onclick={() => {
                                                    if ($settings.defaultResolver === plugin.id) return
                                                    updateSettings('defaultResolver', plugin.id)
                                                }}
                                                class="rounded border-2 px-3 py-2 text-sm hover:bg-white/10 {$settings.defaultResolver ===
                                                plugin.id
                                                    ? 'text-primaryColor'
                                                    : 'text-textColor'}">{plugin.name}</button>
                                        {/each}
                                    </div>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
                <div class="h-full space-y-6">
                    <div class="h-full rounded-lg border border-white/5 p-5">
                        <h4 class="mb-4">Disabled Plugins</h4>
                        <div class="grid h-[90%] space-y-6">
                            {#if $installedResolverPlugins
                                .concat($installedIndexerPlugins)
                                .concat($installedUtilityPlugins).length > 0}
                                <div>
                                    <div class="grid grid-cols-2 gap-x-2 gap-y-2">
                                        {#each $installedResolverPlugins
                                            .concat($installedIndexerPlugins)
                                            .concat($installedUtilityPlugins) as plugin (plugin.id)}
                                            <button
                                                onclick={() => {
                                                    updateSettingsArray('disabledPlugins', plugin.id)
                                                }}
                                                class="rounded border-2 px-3 py-2 text-sm hover:bg-white/10 {$settings.disabledPlugins.includes(
                                                    plugin.id,
                                                )
                                                    ? 'text-primaryColor'
                                                    : 'text-textColor'}">{plugin.name}</button>
                                        {/each}
                                    </div>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    </div>
</section>
