pub mod audio;
pub mod config;
pub mod player;
pub mod shaders;
pub mod subtitles;
pub mod types;
pub mod utils;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(windows)]
pub mod windows;

pub use player::MpvPlayer;
