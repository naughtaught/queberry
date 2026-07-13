export const isVideoFile = (filename: string): boolean => {
    const videoExtensions = ['.mp4', '.mkv', '.avi', '.mov', '.flv', '.webm', 'ogv', 'ogg']
    return videoExtensions.some((ext) => filename.endsWith(ext))
}
