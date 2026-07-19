import { toastNotification } from '$lib/stores/app'
import type { App } from '$lib/types/app'
import { invoke } from '@tauri-apps/api/core'

export const formatError = (
    error: unknown,
    options: App.ErrorOptions = {},
): { success: false; error: App.ErrorDetail } => {
    let message: string
    let stack: string | undefined
    const code = options.code ?? 500

    if (error && typeof error === 'object' && 'success' in error && !error.success) {
        const backendError = error as unknown as { error: App.ErrorDetail }
        return {
            success: false,
            error: {
                ...backendError.error,
                context: options.context ?? backendError.error.context,
            },
        }
    }

    if (error instanceof Error) {
        message = error.message
        stack = error.stack
    } else if (typeof error === 'object' && error !== null) {
        if ('message' in error) {
            message = String(error.message)
        } else {
            try {
                message = JSON.stringify(error)
            } catch {
                message = String(error)
            }
        }
        stack = 'stack' in error ? String(error.stack) : undefined
    } else {
        message = String(error)
    }

    return {
        success: false,
        error: {
            code,
            message,
            stack,
            context: options.context,
        },
    }
}

export const createError = (message: string, code: number, options?: { log?: boolean; display?: boolean }): Error => {
    const error = new Error(message)
    Object.assign(error, { code, ...options })
    return error
}

const logToBackend = async (errorDetail: App.ErrorDetail): Promise<void> => {
    try {
        const payload = JSON.stringify({
            code: errorDetail.code,
            message: errorDetail.message,
            stack: errorDetail.stack,
            context: errorDetail.context,
        })
        await invoke('log_error', { text: payload })
    } catch (e) {
        handleError(e, { display: false })
    }
}

const cleanErrorMessage = (message: string): string => {
    return message
        .replace(/^runtime error:\s*/i, '')
        .replace(/^error:\s*/i, '')
        .trim()
}

export const handleError = <T = unknown>(error: unknown, options: App.ErrorOptions = {}): App.Response<T> => {
    const { log = true, display = true, ...errorOptions } = options
    const formatted = formatError(error, errorOptions)

    if (formatted.error.code === 429) return formatted

    console.error(`[${formatted.error.code}] ${formatted.error.message}`, {
        stack: formatted.error.stack,
        context: formatted.error.context,
        originalError: error,
    })

    if (log) logToBackend(formatted.error).catch(console.debug)

    if (display) {
        toastNotification.show({
            title: 'Error',
            message: cleanErrorMessage(formatted.error.message),
            type: 'error',
        })
    }

    return formatted
}

export const getErrorMessage = (error: unknown): string => {
    if (error instanceof Error) {
        return error.message
    }
    if (typeof error === 'string') {
        return error
    }
    if (error && typeof error === 'object' && 'message' in error) {
        return String(error.message)
    }
    return 'Unknown error'
}
