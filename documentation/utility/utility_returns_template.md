# Example Success Response

## GetIntroTimings

```json
{
    "success": true,
    "data": [
        {
            "tmdb_id": 177221,
            "tvdb_id": 314614,
            "imdb_id": "tt5687612",
            "type": "tv",
            "intro": [
                {
                    "start": null,
                    "end": 30
                }
            ],
            "recap": [
                {
                    "start": 31,
                    "end": 45
                }
            ],
            "credits": [
                {
                    "start": 1680,
                    "end": 1750
                }
            ],
            "preview": [
                {
                    "start": 1640,
                    "end": 1680
                }
            ]
        }
    ]
}
```

> `type` — `tv` or `movie`.
>
> `intro`, `recap`, `credits`, `preview` — `start`/`end` in seconds.
>
> - `intro.start` — seconds, or `null`
> - `recap.start` — seconds, or `null`
> - `credits.end` — seconds, or `null`
> - `preview.end` — seconds, or `null`

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

## UpdateIntroTimings

```json
{
    "success": true,
    "data": []
}
```

# Example Error Response

```json
{
    "success": false,
    "error": "Timer API returned 404 Not Found for URL: https://made.up.stuff/stream/movie/tt1234567.json"
}
```
