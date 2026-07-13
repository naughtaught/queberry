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
    "sources": ["torrents"], // torrents, usenet, etc..
    "types": ["indexer"],
    "cacheless": false,
    "requiresApiKey": "never",  // "never" | "always" | "optional"
    "permissions": {
        "network": ["https://api.your-indexer.com/*"] // Only network permissions permitted
        "allow_private_networks": false // Only needed if using a private network and then set to true
    },
    "apiVersion": "v1",
    "rateLimit": {  // OPTIONAL - overrides the default rate limit for this plugin type
        "maxCalls": 100, // Maximum number of calls allowed
        "windowSeconds": 60 // Time window in seconds (defaults to 60 if omitted)
    },
    "methods": [
        // GetUserInfo IS ONLY REQUIRED IF requiresApiKey is set to optional or always AND your apikey expires
        {
            "interface_method": "GetUserInfo",
            "plugin_method": "", // Your function name - Expected args: [apikey: string]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "GetIndexerSources",
            "plugin_method": "get_sources", // Your function name - Expected args: [apikey: string | null, imdbId: string, season?: int | null, episode?: int | null]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        }
    ]
}
```
