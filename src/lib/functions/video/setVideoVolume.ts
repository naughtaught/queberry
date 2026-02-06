import { handleError, invokeFunction } from '$lib'

export const setVideoVolume = async (
    targetVolume: number,
    currentVolume: number,
    previousVolume: number,
): Promise<{ newValue: number; previousValue: number }> => {
    try {
        if (targetVolume > 100 || targetVolume < 0) throw 'Volume attempted to be set outside of bounds'

        const resp = await invokeFunction('set_volume', { value: targetVolume })

        if (resp.error) throw resp.error

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
