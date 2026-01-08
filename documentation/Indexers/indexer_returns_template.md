# Example Success Response

```json
{
    "success": true,
    "data": [
        {
            "title": "Movie.Title.2023.1080p.BluRay.x264-RELEASE",
            "resolution": "1080p",
            "language": ["en"], // ISO 639-1
            "seeds": 152, // if null then 0
            "size": 8.7,
            "source": "RARBG", // TPB, RARBG, etc...
            "indexer": "YourIndexer",
            "info_hash": "a1b2c3d4e5f6...",
            "filename": "Movie.Title.2023.1080p.BluRay.x264-RELEASE.mkv",
            "video_filters": ["bluray", "remux"],
            "audio_filters": ["dts-hd", "5.1"]
        }
    ],
    "error": null
}
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
