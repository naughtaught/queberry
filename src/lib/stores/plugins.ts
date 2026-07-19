import type { Sql } from '$lib/types/sql'
import type { Plugins } from '$lib/types/plugins'
import { readable, writable } from 'svelte/store'

export const installedIndexerPlugins = writable<Plugins.Plugin[]>([])

export const installedResolverPlugins = writable<Plugins.Plugin[]>([])

export const installedUtilityPlugins = writable<Plugins.Plugin[]>([])

export const enabledIndexerPlugins = writable<Plugins.Plugin[]>([])

export const enabledResolverPlugins = writable<Plugins.Plugin[]>([])

export const enabledUtilityPlugins = writable<Plugins.Plugin[]>([])

export const indexerSources = writable<Plugins.IndexerSources[]>([])

export const transfersInProgress = writable<Plugins.TransferProgress>({})

export const downloadsInProgress = writable<Sql.DownloadProgress[]>([])

export const disabledPlugins = writable<Plugins.Plugin[]>([])

export const excludedTransferStatuses = readable(['downloaded', 'cached', 'completed', 'finished', 'uploading'])
