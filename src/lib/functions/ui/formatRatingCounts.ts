export const formatRatingCounts = (n: number): string | number => {
    if (n < 1e3) return n

    if (n >= 1e3 && n < 1e6) {
        const inK = n / 1e3
        return inK >= 10 ? `${Math.floor(inK)}K` : `${inK.toFixed(1)}K`
    }

    if (n >= 1e6 && n < 1e9) {
        const inM = n / 1e6
        return inM >= 10 ? `${Math.floor(inM)}M` : `${inM.toFixed(1)}M`
    }

    if (n >= 1e9 && n < 1e12) {
        const inB = n / 1e9
        return inB >= 10 ? `${Math.floor(inB)}B` : `${inB.toFixed(1)}B`
    }

    if (n >= 1e12) {
        const inT = n / 1e12
        return inT >= 10 ? `${Math.floor(inT)}T` : `${inT.toFixed(1)}T`
    }

    return n
}
