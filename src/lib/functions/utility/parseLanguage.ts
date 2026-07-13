import { languages } from '$lib/data/languages'

export const languageMaps = (() => {
    const byCode = new Map()
    const byCode2 = new Map()
    const byName = new Map()
    const byAnyCode = new Map()

    languages.forEach((lang) => {
        if (lang.code) byCode.set(lang.code, lang)
        if (lang.code2) byCode2.set(lang.code2, lang)
        if (lang.name) byName.set(lang.name.toLowerCase(), lang)

        if (lang.code) byAnyCode.set(lang.code, lang)
        if (lang.code2) byAnyCode.set(lang.code2, lang)
    })

    return { byCode, byCode2, byName, byAnyCode }
})()

export const getLanguageCodeByName = (name: string): string | null => {
    const language = languageMaps.byName.get(name?.toLowerCase())
    return language?.code ?? null
}

export const getLanguageNameByCode = (code: string): string | null => {
    const language = languageMaps.byAnyCode.get(code)
    return language?.name ?? null
}
