# Example Indexer Manifest

```json
{
    "id": "com.yourname.indexer", // reverse domain notation
    "name": "Your Indexer Name",
    "filename": "your_indexer.wasm",
    "author": "Your Name",
    "homepage": "https://your.app/",
    "description": "Your app description",
    "version": "1.0.0", // semantic versioning
    "sources": ["direct"], // torrents, usenet, direct
    "types": ["utility"],
    "cacheless": false,
    "requiresApiKey": "optional"  // "never" | "always" | "optional"
    "permissions": {
        "network": ["https://api.your-indexer.com/*"] // Only network permissions permitted
        "allow_private_networks": false // Only needed if using a private network and then set to true
    },
    "apiVersion": "v1",
        "rateLimit": { // OPTIONAL - overrides the default rate limit for this plugin type
        "maxCalls": 100, // Maximum number of calls allowed
        "windowSeconds": 60 // Time window in seconds (defaults to 60 if omitted)
    },
    "methods": [
        // GetUserInfo IS ONLY REQUIRED IF requiresApiKey is set to optional or always AND your apikey expires
        {
            "interface_method": "GetUserInfo",
            "plugin_method": "", // Your function name - Expected args: [apikey: string | null]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "GetIntroTimings",
            "plugin_method": "", // Your function name - Expected args: [apikey: string | null, imdbId: string, tmdbId: number | null, tvdbId: number | null, duration: number (seconds),   season: int | null, episode: int | null, type: tv | movie
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "UpdateIntroTimings",
            "plugin_method": "", // Your function name - Expected args: [apikey: string | null, imdbId: string, tmdbId: number | null, tvdbId: number | null, segment: intro | recap, preview | credits,  season: int | null, episode: int | null, duration: number (seconds) , start: number (seconds) | null, end: number (seconds) | null, type: tv | movie]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        }
    ]
}
```
