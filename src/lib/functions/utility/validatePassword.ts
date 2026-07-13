import { MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH } from "$lib/stores/app";

export const validatePassword = (
    password: string,
    passwordConfirmation: string | null = null,
): { success: boolean; error: string | null } => {
    if (!password || password.trim() === '') {
        return {
            success: false,
            error: 'Password is required.',
        }
    }

    if (password.trim() !== password) {
        return {
            success: false,
            error: 'Password cannot start or end with whitespace.',
        }
    }

    if (password.length < MIN_PASSWORD_LENGTH) {
        return {
            success: false,
            error: `Password must be at least ${MIN_PASSWORD_LENGTH} characters.`,
        }
    }

    if (password.length > MAX_PASSWORD_LENGTH) {
        return {
            success: false,
            error: `Password must be at most ${MAX_PASSWORD_LENGTH} characters.`,
        }
    }

    if (passwordConfirmation !== null) {
        if (passwordConfirmation.trim() === '') {
            return {
                success: false,
                error: 'Password confirmation is required.',
            }
        }

        if (password !== passwordConfirmation) {
            return {
                success: false,
                error: 'Password & password confirmation do not match.',
            }
        }
    }

    return {
        success: true,
        error: null,
    }
}
