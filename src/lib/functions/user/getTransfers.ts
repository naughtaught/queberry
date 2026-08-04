import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { enabledResolverPlugins, transfersInProgress } from '$lib/stores/plugins'
import type { Plugins } from '$lib/types/plugins'
import { handleError } from '$lib/functions/errors/errorHandling'
import { get } from 'svelte/store'
import { fetchTransfers } from '../plugins/fetchTransfers'

const addTransferToDb = async (transfer: Plugins.TransferData, resolverId: string): Promise<void> => {
    try {
        const resp = await invokeFunction('create_transfer', {
            hash: transfer.infohash,
            transfer: {
                transferId: +transfer.id,
                progress: 0,
                status: transfer.status,
                resolver: resolverId,
                filename: transfer.filename,
            },
        })

        if (!resp.success) throw resp.error

        transfersInProgress.update((transfers) => ({
            ...transfers,
            [transfer.infohash]: {
                transferId: +transfer.id,
                filename: transfer.filename,
                progress: transfer.progress,
                hash: transfer.infohash,
                status: transfer.status,
                resolver: resolverId,
                speed: 0,
            },
        }))
    } catch (error) {
        handleError(error)
    }
}

export const getTransfers = async (): Promise<void> => {
    try {
        const transfers = await invokeFunction('list_transfers', {})
        if (!transfers.success) throw transfers.error

        const transfersObject: Plugins.TransferProgress = {}

        for (const transfer of transfers.data) {
            transfersObject[transfer.hash] = {
                transferId: transfer.transferId,
                progress: transfer.progress,
                filename: transfer.filename,
                hash: transfer.hash,
                status: 'checking',
                resolver: transfer.resolver,
                speed: 0,
            }
        }

        transfersInProgress.set(transfersObject)

        const dbTransfers = Object.values(get(transfersInProgress))
        const resolverPlugins = get(enabledResolverPlugins)

        const resolverTransfersWithResolver = await Promise.all(
            resolverPlugins.map(async (resolver) => {
                const resolverTransfers = await fetchTransfers(resolver)
                return resolverTransfers.map((transfer) => ({
                    transfer,
                    resolverId: resolver.id,
                }))
            }),
        )
        const allResolverTransfers = resolverTransfersWithResolver.flat()

        const newTransfers = allResolverTransfers.filter(
            ({ transfer }) =>
                transfer.status !== 'cached' &&
                !dbTransfers.some(
                    (dbTransfer) =>
                        dbTransfer.transferId.toString() === transfer.id || dbTransfer.hash === transfer.infohash,
                ),
        )

        await Promise.all(newTransfers.map(({ transfer, resolverId }) => addTransferToDb(transfer, resolverId)))

        const transfersToRemove = dbTransfers.filter(
            (dbTransfer) =>
                !allResolverTransfers.some(
                    ({ transfer }) =>
                        transfer.id === dbTransfer.transferId.toString() || transfer.infohash === dbTransfer.hash,
                ),
        )

        await Promise.all(
            transfersToRemove.map(async (transfer) => {
                const resp = await invokeFunction('delete_transfer', { hash: transfer.hash })

                if (!resp.success) throw resp.error

                transfersInProgress.update((current) => {
                    const { [transfer.hash]: _, ...rest } = current
                    return rest
                })
            }),
        )
    } catch (error) {
        handleError(error, {
            display: false,
        })
    }
}
