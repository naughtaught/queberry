export const abbreviateName = (fullName: string): string => {
    const parts = fullName.trim().split(' ')
    if (parts.length === 1) return fullName

    const firstName = parts[0]
    const lastName = parts[parts.length - 1]

    if (firstName.length === 1 || firstName.endsWith('.')) return fullName

    return `${firstName[0]}. ${lastName}`
}
