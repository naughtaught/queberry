import { type Api } from '$lib'

export const handleError = (error: Api.ErrorDetail): Api.ApiResponse => {
    console.error(`Error ${error.code}: ${error.message} - ${error.stack}`)

    return {
        success: false,
        data: null,
        error,
    }
}
