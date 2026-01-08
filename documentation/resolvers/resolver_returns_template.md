# Example GetUserInfo Response

```json
{
    "success": true,
    "data": [
        {
            "user_id": "651s3452dvsf...", // string
            "premium_until": "2028-12-31" // YYYY-MM-DD
        }
    ],
    "error": null
}
```

# Example CheckIfCached Response

```json
{
    "success": true,
    "data": [
        {
            "hash": "sdgf32sdwfdfg...", // string
            "cached": true, // boolean
            "files": [
                { "name": "TV.Title.2023.1080p.S1E1.BluRay.x264-RELEASE.mkv" },
                { "name": "TV.Title.2023.1080p.S1E2.BluRay.x264-RELEASE.mkv" }
            ] // {name: string}[]
        }
    ],
    "error": null
}
```

# Example GetCachedFiles Response

```json

```

# Example AddFilesToCache Response

```json

```

# Example GetTransferInfo Response

```json

```

# Example CancelTransfer Response

```json

```

# Example UnrestrictLink Response

```json
{
    "success": true,
    "data": [
        {
            "link": "651s3452dvsf...",
            "filename": "2028-12-31"
            "mime_type":
            "file_size":
            "streamable":
        }
    ],
    "error": null
}
```

# Example GetDownloadLink Response

```json

```

# Example GetTorrentList Response

```json

```

# Example Error Response

```json
{
    "success": false,
    "data": null,
    "error": {
        "code": 404,
        "message": "API returned 404 Not Found",
        "stack": "https://api.example.com/search?imdb=tt1234567"
    }
}
```
