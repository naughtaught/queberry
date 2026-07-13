# Plugin API Reference

## Response Envelope

All responses follow one of two shapes:

### Success

```json
{ "success": true, "data": <see per-function docs below> }
```

### Error

```json
{ "success": false, "error": "Human-readable error message" }
```

---

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

> `premium_until`:
>
> - timestamp (unix, UTC) — active premium, known expiry
> - `0` — no expiration applies (no premium tier, or lifetime/infinite premium)
> - `null` — premium existed but has expired

---

## CheckIfCached

**Args:** `[api_key: string, hashes: string[]]`

```json
{
    "success": true,
    "data": [
        {
            "hash": "sdgf32sdwfdfg...",
            "cached": true,
            "files": [
                { "name": "TV.Title.2023.1080p.S1E1.BluRay.x264-RELEASE.mkv" },
                { "name": "TV.Title.2023.1080p.S1E2.BluRay.x264-RELEASE.mkv" }
            ]
        }
    ]
}
```

> One entry is returned per input hash. `cached: false` entries will have an empty `files` array.

---

## FetchVideoUrl

**Args:** `[api_key: string, magnet_or_hash: string, season?: number, episode?: number]`

```json
{
    "success": true,
    "data": {
        "download_link": "https://nexus-074.cnam.tb-cdn.io/dld/1c3b6052-b502-40e7-b280-ea63ca800e9c?token=b11e149e-7570-4f56-ad1f-e4d05c863f27",
        "filename": "Interstellar.2014.IMAX.UHD.BluRay.2160p.DDP.5.1.DV.HDR.x265-hallowed.mkv",
        "files": [
            {
                "link": "&torrent_id=15957770&file_id=0",
                "filename": "Interstellar.2014.IMAX.UHD.BluRay.2160p.DDP.5.1.DV.HDR.x265-hallowed.mkv",
                "size": 21229500651
            }
        ]
    }
}
```

> Returns `null` data if no video files are found in the torrent.
> `files` contains all video files found; `download_link` and `filename` point to the selected one.
> `size` is in bytes.

---

## UnrestrictLink

**Args:** `[api_key: string, link: string]`

```json
{
    "success": true,
    "data": {
        "link": "https://nexus-074.cnam.tb-cdn.io/dld/..."
    }
}
```

---

## AddFilesToCache

**Args:** `[api_key: string, magnet_or_hash: string]`

```json
{
    "success": true,
    "data": {
        "id": "2345234",
        "status": "downloading",
        "progress": 0
    }
}
```

> `progress` is always `0` on creation.
> On error, `success` is still `true` but `data` will contain `error_type` and `error_message`:

```json
{
    "success": true,
    "data": {
        "id": "0",
        "status": "error",
        "progress": 0,
        "error_type": "ACTIVE_LIMIT | MONTHLY_LIMIT | SERVER_ERROR | RATE_LIMIT | INVALID_API_KEY | RESOURCE_NOT_FOUND | CREATE_TORRENT_FAILED | NO_TORRENT_DATA",
        "error_message": "..."
    }
}
```

---

## GetTorrentList

**Args:** `[api_key: string, torrent_id: string]`

```json
{
    "success": true,
    "data": {
        "id": "2345234",
        "progress": 88,
        "speed": 512.0,
        "status": "downloading"
    }
}
```

> `progress` is `0–100`. `speed` is in Mbps (float).
> Possible `status` values: `"downloading"`, `"queued"`, `"paused"`, `"finished"`, `"error"`, or a raw state string for any unrecognised states.

---

## CancelTransfer

**Args:** `[api_key: string, torrent_id: string]`

```json
{
    "success": true,
    "data": null
}
```
