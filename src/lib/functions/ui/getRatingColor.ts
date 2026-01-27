export const getRatingColor = (avg: number, prefix: string): string => {
    if (!avg) return `${prefix}-black`
    if (avg >= 90) return `${prefix}-rating-90`
    if (avg >= 80) return `${prefix}-rating-80`
    if (avg >= 70) return `${prefix}-rating-70`
    if (avg >= 60) return `${prefix}-rating-60`
    if (avg >= 50) return `${prefix}-rating-50`
    if (avg >= 40) return `${prefix}-rating-40`
    if (avg >= 30) return `${prefix}-rating-30`
    if (avg >= 20) return `${prefix}-rating-20`
    if (avg >= 10) return `${prefix}-rating-10`
    return `${prefix}-rating-0`
}
