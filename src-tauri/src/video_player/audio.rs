use crate::errors::{AppError, Result};
use libmpv2::Mpv;

pub fn set_audio_channel(mpv: &Mpv, audio_channel: &str) -> Result<()> {
    let normalized_channel = match audio_channel.to_lowercase().as_str() {
        "7.1" => "7.1",
        "6.1" => "6.1",
        "6.0" => "6.0",
        "5.1" => "5.1",
        "5.0" => "5.0",
        "4.1" => "4.1",
        "4.0" => "4.0",
        "3.1" => "3.1",
        "3.0" => "3.0",
        "2.1" => "2.1",
        "2.0" | "stereo" => "stereo",
        "auto" | "original" => "auto",
        _ => {
            return Err(AppError::Runtime(format!(
                "Unsupported audio channel: {}",
                audio_channel
            )));
        }
    };

    let _ = mpv.command("no-osd", &["af", "remove", "loudnorm"]);

    mpv.set_property("audio-channels", normalized_channel)
        .map_err(|e| {
            AppError::Runtime(format!(
                "Failed to set audio channel to {}: {}",
                normalized_channel, e
            ))
        })?;

    if normalized_channel == "stereo" || normalized_channel == "2.1" {
        if let Err(e) = mpv.command("no-osd", &["af", "add", "loudnorm=I=-16:TP=-1.5:LRA=11"]) {
            log::debug!("Could not add loudnorm filter during initialization: {}", e);
        }
    }

    Ok(())
}
