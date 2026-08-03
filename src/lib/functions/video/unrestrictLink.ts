import type { Plugins } from '$lib/types/plugins'
import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { handleError } from '$lib/functions/errors/errorHandling'
import { checkMethodApi } from '$lib/functions/plugins/checkMethodApi'

export const unrestrictLink = async (
    apikey: string | null | undefined,
    link: string,
    plugin: Plugins.Plugin,
): Promise<string | null> => {
    try {
        checkMethodApi(plugin, 'UnrestrictLink')

        const response = await invokeFunction('call_plugin_method', {
            pluginName: plugin.id,
            methodName: 'UnrestrictLink',
            args: [apikey ?? null, link],
        })

        if (!response.success) throw response.error

        return response.data.link ?? null
    } catch (error) {
        handleError(error, { display: false })
        return null
    }
}
