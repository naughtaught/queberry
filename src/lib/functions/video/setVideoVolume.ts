import { invokeFunction } from '$lib'

export const setVideoVolume = async (
    targetVolume: number,
    currentVolume: number,
    previousVolume: number,
): Promise<{ newValue: number; previousValue: number }> => {
    const response = await invokeFunction('set_volume', { value: targetVolume })
    if (response.success) {
        return {
            newValue: response.data.value,
            previousValue: currentVolume,
        }
    }

    return {
        newValue: currentVolume,
        previousValue: previousVolume,
    }
}
