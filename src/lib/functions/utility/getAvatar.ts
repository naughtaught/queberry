import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { join } from '@tauri-apps/api/path'

export const getAvatar = async (name: string): Promise<string> => {
    if (name.startsWith('http://') || name.startsWith('asset://')) return name
    const dir: string = await invoke('get_avatars_dir_path')

    return convertFileSrc(await join(dir, name))
}
