import type { Video } from '$lib/types/video'

const EPISODE_PATTERNS = [
    { regex: /[sS](\d+)[eE](\d+)/, label: 'sXXeYY' },
    { regex: /(\d+)x(\d+)/, label: 'XxYY' },
    { regex: /season\s*(\d+)\s*episode\s*(\d+)/i, label: 'written' },
    { regex: /(\d+)\.(\d+)(?!\d)/, label: 'dot' },
    { regex: /(?<![.\d])(\d)(\d{2})(?!\d)/, label: 'compact' },
]

export const parseFilenameForEpisode = (
    seasonNum: number,
    episodeNum: number,
    files: Video.Files[],
): Video.Files | null => {
    for (const file of files) {
        const filename = file.filename.split('/').pop()?.toLowerCase() ?? file.filename.toLowerCase()

        for (const { regex } of EPISODE_PATTERNS) {
            const match = filename.match(regex)
            if (!match) continue

            const season = parseInt(match[1], 10)
            const episode = parseInt(match[2], 10)

            if (season === seasonNum && episode === episodeNum) {
                return file
            }
        }
    }

    return null
}
