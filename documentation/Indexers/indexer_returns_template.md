# Example Success Response

```json
{
    "success": true,
    "data": [
        {
            "title": "Movie.Title.2023.1080p.BluRay.x264-RELEASE",
            "resolution": "2160p",
            "language": ["en", "es"],
            "seeds": 152,
            "size": 8.7,
            "source": "RARBG",
            "indexer": "Torrentio",
            "info_hash": "a1b2c3d4e5f6789012345678901234567890ab",
            "filename": "Movie.Title.2023.1080p.BluRay.x264-RELEASE.mkv",
            "video_filters": ["HDR10", "Dolby Vision", "Remux"],
            "video_details": {
                "codec": "HEVC",
                "bitrate": "62.4mbps",
                "framerate": "23.976fps"
            },
            "audio_details": {
                "codec": "DTS-HD MA",
                "channels": "7.1",
                "sampling_rate": "48khz",
                "bitrate": "1509kbps"
            },
            "tags": ["IMAX", "10-bit", "Director's Cut", "BT.2020"]
        }
    ]
}
```

> `language` — ISO 639-1 codes.
>
> `seeds` — `0` if not available.
>
> `size` — size in GB.
>
> `source` — release group or source name.
>
> `indexer` — indexer name.
>
> `video_filters` — video enhancements.
>
> `video_details.codec` — AV1, HEVC, H.264, VP9, etc.
>
> `video_details.bitrate` — optional, if available.
>
> `video_details.framerate` — optional, if available.
>
> `audio_details.codec` — TrueHD, DTS, AC-3, AAC, etc.
>
> `audio_details.channels` — 7.1, 5.1, 2.0, etc.
>
> `audio_details.sampling_rate` — optional, if available.
>
> `audio_details.bitrate` — optional, if available.
>
> `tags` — additional metadata.

## GetUserInfo

**Args:** `[api_key: string]`

```json
{
    "success": true,
    "data": [
        {
            "user_id": "651s3452dvsf...",
            "premium_until": 1772400488
        }
    ]
}
```

> `user_id` — `0` if no account data exists.
>
> `premium_until`:
>
> - timestamp (unix, UTC) — active premium, known expiry
> - `0` — no expiration applies (no premium tier, or lifetime/infinite premium)
> - `null` — premium existed but has expired

# Example Error Response

```json
{
    "success": false,
    "error": "Indexer API returned 404 Not Found for URL: https://made.up.stuff/stream/movie/tt1234567.json"
}
```
