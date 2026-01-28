import { invoke } from '@tauri-apps/api/core'
import { handleError, type Api } from '$lib'

export const invokeFunction = async (emit: string, args: Record<string, unknown>): Promise<Api.ApiResponse> => {
    try {
        const response: Api.ApiResponse = await invoke(emit, args)
        if (response.success) {
            return response
        } else {
            handleError(response.error!)
            return response
        }
    } catch (error) {
        const errorDetail: Api.ErrorDetail = {
            code: 500,
            message: error instanceof Error ? error.message : String(error),
            stack: error instanceof Error ? error.stack : undefined,
        }
        return handleError(errorDetail)
    }
}
