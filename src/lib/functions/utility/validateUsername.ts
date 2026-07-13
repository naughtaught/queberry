export const validateUsername = (username: string): { success: boolean; error: string | null } => {
    if (!username || username.trim() === '') {
        return {
            success: false,
            error: 'Username is required.',
        }
    }

    return {
        success: true,
        error: null,
    }
}
