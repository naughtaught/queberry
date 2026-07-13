# Example Resolver Manifest

```json
{
    "id": "app.nachos.resolver", // reverse domain notation
    "name": "Nachos Resolver",
    "filename": "nachos_plugin.wasm",
    "author": "NACHO!",
    "homepage": "https://nacho.app/",
    "description": "Nacho torrent resolver plugin for resolving torrents by magnet link",
    "version": "1.0.0", // semantic versioning
    "sources": ["debrid"], // debrid, usenet, etc..
    "types": ["resolver"],
    "cacheless": false, // boolean | only true if resolver does not have a check if cached method (ex. Real Debrid)
    "requiresApiKey": "always", // "never" | "always" | "optional"
    "permissions": {
        "network": ["https://api.nacho.app/v1/api/*"], // Only network permissions permitted
        "allow_private_networks": false // Only needed if using a private network and then set to true
    },
    "apiVersion": "v1",
    "rateLimit": {
        // OPTIONAL - overrides the default rate limit for this plugin type
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
            "interface_method": "CheckIfCached",
            "plugin_method": "", // Expected args: [apikey: string | null, hashes: string[]]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "FetchVideoUrl",
            "plugin_method": "", // Expected args: [apikey: string | null, magnetlink: string, season?: int | null, episode?: int | null]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "UnrestrictLink",
            "plugin_method": "", // Expected args: [apikey: string | null, link: string]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "AddFilesToCache",
            "plugin_method": "", // Expected args: [apikey: string | null, link: string]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "GetTransferInfo",
            "plugin_method": "", // Expected args: [apikey: string | null, torrent_id: string]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "GetTorrentList",
            "plugin_method": "", // Expected args: [apikey: string | null, torrent_id: string]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        },
        {
            "interface_method": "CancelTransfer",
            "plugin_method": "", // Expected args: [apikey: string | null]
            "requires_api_key": false // boolean if true then top level "requiresApiKey" must not be never
        }
    ]
}
```
