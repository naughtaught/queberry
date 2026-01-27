// Types
export type { Api } from './types/api'
export type { App } from './types/app'

// Components/DB
export { supabase } from './components/db/supabaseClient.ts'

// Components/Header
export { default as Header } from './components/Header/Header.svelte'
export { default as HeaderLink } from './components/Header/HeaderLink.svelte'
export { default as HeaderButton } from './components/Header/HeaderButton.svelte'

// Components/Inputs
export { default as Slider } from './components/inputs/Slider.svelte'
export { default as UserRating } from './components/inputs/UserRating.svelte'
export { default as UserRatingButton } from './components/inputs/UserRatingButton.svelte'
export { default as MediaStateBaseButton } from './components/inputs/MediaStateBaseButton.svelte'
export { default as HideButton } from './components/inputs/HideButton.svelte'
export { default as CollectionButton } from './components/inputs/CollectionButton.svelte'

// Components/VideoPlayer
export { default as VideoControls } from './components/videoplayer/VideoControls.svelte'
export { default as PlayButton } from './components/videoplayer/PlayButton.svelte'
export { default as SeekButton } from './components/videoplayer/SeekButton.svelte'
export { default as SeekBar } from './components/videoplayer/SeekBar.svelte'
export { default as Volume } from './components/videoplayer/Volume.svelte'
export { default as VideoHeader } from './components/videoplayer/VideoHeader.svelte'
export { default as AudioChannels } from './components/videoplayer/AudioChannels.svelte'
export { default as SelectModal } from './components/videoplayer/SelectModal.svelte'
export { default as SubtitleTrackButton } from './components/videoplayer/SubtitleTrackButton.svelte'
export { default as AudioTrackButton } from './components/videoplayer/AudioTrackButton.svelte'
export { default as VideoSettingsButton } from './components/videoplayer/VideoSettingsButton.svelte'
export { default as VideoMenuButton } from './components/videoplayer/VideoMenuButton.svelte'

// Functions/UI
export { toggleFullscreen } from './functions/ui/toggleFullscreen.ts'
export { minimizeApp } from './functions/ui/minimizeApp.ts'
export { closeApp } from './functions/ui/closeApp.ts'
export { getRatingColor } from './functions/ui/getRatingColor.ts'

// Functions/User
export { updateLocalMediaStates } from './functions/user/updateLocalMediaStates.ts'

// Stores
export { handleError } from './stores/errorHandling'
export { videoMetadata, videoState, defaultVideoMetadata, defaultVideoState, audioChannelOptions } from './stores/video'
export { settings, user } from './stores/user'
export { appState } from './stores/app'
