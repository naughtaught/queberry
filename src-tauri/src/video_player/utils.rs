#[cfg(windows)]
pub use crate::video_player::windows::{get_window_handle_id, platform_specific_init};

#[cfg(target_os = "linux")]
pub use crate::video_player::linux::{get_window_handle_id, platform_specific_init};

#[cfg(target_os = "macos")]
pub use crate::video_player::macos::{get_window_handle_id, platform_specific_init};
