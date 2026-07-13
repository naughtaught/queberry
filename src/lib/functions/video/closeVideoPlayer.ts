import { invokeFunction } from '../api/invokeFunction'
import { handleError } from '../errors/errorHandling'

export const closeVideoPlayer = async (): Promise<void> => {
    try {
        const resp = await invokeFunction('close_video_player', {})
        if (!resp.success) throw resp.error
    } catch (error) {
        handleError(error, {
            context: 'closing the video failed',
        })
    }
}
