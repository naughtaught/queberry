import { MAX_PIN_LENGTH, MIN_PIN_LENGTH } from '$lib/stores/app'

export const validatePin = (pin: string): { success: boolean; error: string | null } => {
    if (!pin || pin.trim() === '') {
        return {
            success: false,
            error: 'PIN is required.',
        }
    }

    if (pin !== pin.trim()) {
        return {
            success: false,
            error: 'PIN cannot start or end with spaces.',
        }
    }

    if (/\s/.test(pin)) {
        return {
            success: false,
            error: 'PIN cannot contain spaces.',
        }
    }

    if (pin.length < MIN_PIN_LENGTH) {
        return {
            success: false,
            error: `PIN must be at least ${MIN_PIN_LENGTH} characters.`,
        }
    }

    if (pin.length > MAX_PIN_LENGTH) {
        return {
            success: false,
            error: `PIN must be at most ${MAX_PIN_LENGTH} characters.`,
        }
    }

    return {
        success: true,
        error: null,
    }
}
