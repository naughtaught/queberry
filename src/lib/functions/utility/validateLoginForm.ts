import { validateEmail } from '$lib/functions/utility/validateEmail'
import { validatePassword } from '$lib/functions/utility/validatePassword'
import { validatePin } from '$lib/functions/utility/validatePin'
import { validateUsername } from '$lib/functions/utility/validateUsername'

export const validateLoginForm = (
    email: string,
    password: string,
    pin: string,
    passwordConfirmation: string | null = null,
    username: string | null = null,
): { success: boolean; error: string | null } => {
    const isEmailValid = validateEmail(email)
    if (isEmailValid.error) {
        return {
            success: false,
            error: isEmailValid.error,
        }
    }
    const isPasswordValid = validatePassword(password, passwordConfirmation)
    if (isPasswordValid.error) {
        return {
            success: false,
            error: isPasswordValid.error,
        }
    }
    const isPinValid = validatePin(pin)
    if (isPinValid.error) {
        return {
            success: false,
            error: isPinValid.error,
        }
    }
    if (username) {
        const isUsernameValid = validateUsername(username)
        if (isUsernameValid.error) {
            return {
                success: false,
                error: isUsernameValid.error,
            }
        }
    }

    return {
        success: true,
        error: null,
    }
}
