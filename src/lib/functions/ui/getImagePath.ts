export const getImagePath = (image: string | null | undefined, size = 'original'): string | null => {
    if (!image) return null

    if (image.startsWith('http')) return image

    const baseUrl = 'https://image.tmdb.org/t/p/'

    return `${baseUrl}${size}/${image}`
}
