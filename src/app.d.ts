import type { ApiResponse, ErrorDetail } from '$lib/types/sql'
import 'unplugin-icons/types/svelte'

declare global {
    namespace App {
        interface Response extends ApiResponse {
            success: false
            data: null
            error: ErrorDetail | null
        }

        interface PageData extends ApiResponse {}

        // interface Locals {}
        // interface PageState {}
        // interface Platform {}
    }
    namespace svelteHTML {
        interface HTMLAttributes<T> {
            onenterViewport?: (event: CustomEvent) => void
        }
        interface HTMLAttributes<T> {
            onclickOutside?: CompositionEventHandler
        }
    }
}

export {}
