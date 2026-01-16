import type { ApiResponse, ErrorDetail } from '$lib/types/api'

declare global {
    namespace App {
        interface Response extends ApiResponse {
            success: false
            data: null
            error: ErrorDetail | null
        }

        interface PageData {
            success: boolean
            error: ErrorDetail | null
        }

        // interface Locals {}
        // interface PageState {}
        // interface Platform {}
    }
}

export {}
