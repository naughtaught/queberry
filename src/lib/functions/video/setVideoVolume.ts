import { invokeFunction } from '$lib/functions/api/invokeFunction'
import { createError, handleError } from '$lib/functions/errors/errorHandling'

export const setVideoVolume = async (
    targetVolume: number,
    currentVolume: number,
    previousVolume: number,
): Promise<{ newValue: number; previousValue: number }> => {
    try {
        if (targetVolume > 100 || targetVolume < 0) {
            throw createError(`Volume attempted to be set outside of bounds`, 400, {
                log: false,
                display: false,
            })
        }

        const resp = await invokeFunction('set_volume', { value: targetVolume })

        if (!resp.success) throw resp.error

        return {
            newValue: resp.data.value,
            previousValue: currentVolume,
        }
    } catch (error) {
        handleError(error, {
            context: 'setting the video volume failed',
        })

        return {
            newValue: currentVolume,
            previousValue: previousVolume,
        }
    }
}
