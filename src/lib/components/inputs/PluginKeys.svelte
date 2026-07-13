<script lang="ts">
    import { invokeFunction } from '$lib/functions/api/invokeFunction'
    import { handleError } from '$lib/functions/errors/errorHandling'
    import { setEnabledPlugins } from '$lib/functions/plugins/setEnabledPlugins'
    import {
        enabledResolverPlugins,
        installedIndexerPlugins,
        installedResolverPlugins,
        installedUtilityPlugins,
    } from '$lib/stores/plugins'
    import { settings, user } from '$lib/stores/user'
    import type { Plugins } from '$lib/types/plugins'
    import Select from '$lib/components/inputs/Select.svelte'

    const { func } = $props()

    let plugin: Plugins.Plugin | null = $derived(null)
    let newKey: string | null = $derived(null)
    let currentKey = $derived((plugin as Plugins.Plugin | null)?.apikey ?? null)
    const pluginsRequiringKeys = $derived(
        $installedResolverPlugins
            .concat($installedIndexerPlugins)
            .concat($installedUtilityPlugins)
            .filter((x) => x.requiresApiKey !== 'never'),
    )
    const pluginOptions = $derived(pluginsRequiringKeys.map((x) => ({ name: x.name, value: x })))

    $effect(() => {
        if (plugin) newKey = null
    })

    const updateKey = async (): Promise<void> => {
        if (!$user || !plugin || !newKey || newKey.trim().length === 0) return

        try {
            const hasGetUserInfo = plugin.methods.some((m) => m.interfaceMethod === 'GetUserInfo')

            let expiresAt

            if (hasGetUserInfo) {
                const getUserInfoResponse = await invokeFunction('call_plugin_method', {
                    pluginName: plugin.id,
                    methodName: 'GetUserInfo',
                    args: [newKey.trim()],
                })

                if (!getUserInfoResponse.success) throw getUserInfoResponse.error

                expiresAt = getUserInfoResponse.data.premium_until
            } else {
                expiresAt = 0
            }

            if (currentKey) {
                const response = await invokeFunction('update_key', {
                    plugin: plugin.id,
                    key: newKey.trim(),
                    expiration: expiresAt,
                })

                if (!response.success) throw response.error
            } else {
                const response = await invokeFunction('create_key', {
                    plugin: plugin.id,
                    key: newKey.trim(),
                    expiration: expiresAt,
                })

                if (!response.success) throw response.error
            }

            const pluginsResp = await setEnabledPlugins()
            if (!pluginsResp.success) throw pluginsResp.error

            if (!$settings.defaultResolver) {
                func('defaultResolver', $enabledResolverPlugins[0].id)
            }

            plugin = null
            newKey = null
        } catch (error) {
            handleError(error)
        }
    }

    const deleteKey = async (): Promise<void> => {
        if (!$user || !plugin) return

        try {
            const response = await invokeFunction('delete_key', {
                plugin: plugin.id,
            })

            if (!response.success) throw response.error

            const pluginsResp = await setEnabledPlugins()
            if (!pluginsResp.success) throw pluginsResp.error

            if ($settings.defaultResolver === plugin?.id) {
                const defaultResolver = $enabledResolverPlugins[0] ? $enabledResolverPlugins[0].id : null
                func('defaultResolver', defaultResolver)
            }

            plugin.apikey = null
            plugin = null
            currentKey = null
            newKey = null
        } catch (error) {
            handleError(error)
        }
    }

    $effect(() => {
        if (currentKey) newKey = currentKey
    })
</script>

<div class="flex gap-x-2">
    <div class="flex max-w-52 flex-1 gap-x-2">
        <div class="w-full">
            <Select maxWidth="max-w-full" options={pluginOptions} bind:activeOption={plugin} name="" />
        </div>
    </div>
    <div class="flex w-78 items-end">
        <input type="text" class="h-8.5 flex-1 rounded bg-slate-800 px-2 py-1 text-slate-200" bind:value={newKey} />
    </div>
    <div class="flex items-end gap-x-2">
        {#if currentKey?.trim() !== newKey?.trim() && plugin}
            <button
                onclick={updateKey}
                class="h-8.5 rounded-lg bg-slate-800 px-6 font-bold shadow-lg hover:text-primaryColor">Submit</button>
        {/if}
        {#if currentKey?.trim() && currentKey.trim() === newKey?.trim() && plugin}
            <button
                onclick={deleteKey}
                disabled={newKey !== currentKey}
                class="h-8.5 rounded-lg bg-slate-800 px-6 font-bold shadow-lg hover:text-red-500">
                Delete
            </button>
        {/if}
    </div>
</div>
