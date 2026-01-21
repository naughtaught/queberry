import { type Api } from '$lib'

export const handleError = (error: Api.ErrorDetail): void => {
    console.error(`Error ${error.code}: ${error.message} - ${error.stack}`)
}
