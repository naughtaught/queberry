import type { Api } from '$lib/types/api'

export const handleError = <T = unknown>(error: unknown, options: Api.ErrorOptions = {}): Api.ApiResponse<T> => {
    const errorDetail: Api.ErrorDetail = {
        code: options.code || 500,
        message: error instanceof Error ? error.message : String(error),
        stack: error instanceof Error ? error.stack : undefined,
        context: options.context,
    }

    console.error(`Error ${errorDetail.code}: ${errorDetail.message}`, {
        stack: errorDetail.stack,
        context: errorDetail.context,
        originalError: options.originalError,
    })

    return {
        success: false,
        data: null,
        error: errorDetail,
    }
}
