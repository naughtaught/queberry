export const validateEmail = (email: string): { success: boolean; error: string | null } => {
    const minLength = 6
    const maxLength = 254

    if (!email || email.trim() === '') {
        return {
            success: false,
            error: 'Email is required.',
        }
    }

    const trimmedEmail = email.trim()

    if (trimmedEmail.length < minLength) {
        return {
            success: false,
            error: `Email must be at least 6 characters.`,
        }
    }

    if (trimmedEmail.length > maxLength) {
        return {
            success: false,
            error: `Email must be at most 254 characters.`,
        }
    }

    if ((trimmedEmail.match(/@/g) || []).length !== 1) {
        return {
            success: false,
            error: 'Email must contain exactly one @ symbol.',
        }
    }

    const [localPart, domainPart] = trimmedEmail.split('@')

    if (!localPart || localPart.length === 0) {
        return {
            success: false,
            error: 'Email must have at least one character before @.',
        }
    }

    if (!domainPart || domainPart.length === 0) {
        return {
            success: false,
            error: 'Email must have at least one character after @ and before the dot.',
        }
    }

    if (!domainPart.includes('.')) {
        return {
            success: false,
            error: 'Domain must contain a dot (.)',
        }
    }

    const lastDotIndex = domainPart.lastIndexOf('.')
    if (lastDotIndex === 0) {
        return {
            success: false,
            error: 'Domain must have at least one character between @ and the last dot.',
        }
    }

    const tld = domainPart.split('.').pop() || ''
    if (tld.length < 2) {
        return {
            success: false,
            error: 'The part after the last dot must be at least 2 characters.',
        }
    }

    if (!/^[a-zA-Z]+$/.test(tld)) {
        return {
            success: false,
            error: 'The part after the last dot must contain only letters.',
        }
    }

    if (domainPart.includes('..')) {
        return {
            success: false,
            error: 'Domain cannot contain consecutive dots.',
        }
    }

    if (!/^[a-zA-Z0-9.-]+$/.test(domainPart)) {
        return {
            success: false,
            error: 'Domain contains invalid characters.',
        }
    }

    return {
        success: true,
        error: null,
    }
}
