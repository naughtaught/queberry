# Example Resolver Manifest

```json
{
    "id": "app.torbox.resolver", // reverse domain notation
    "name": "Torbox Resolver",
    "filename": "torbox_plugin.wasm",
    "author": "Anonymouse",
    "homepage": "https://torbox.app/",
    "description": "Zilean DMM torrent indexer plugin for searching torrents by IMDb ID",
    "version": "1.0.0", // semantic versioning
    "sources": ["debrid"], // debrid, usenet, etc..
    "types": ["resolver"],
    "cacheless": false, // boolean | only true if resolver does not have a check if cached method (ex. Real Debrid)
    "permissions": {
        "network": ["https://api.torbox.app/v1/api/*"] // Only network permissions permitted
        "allow_private_networks": false // Only needed if using a private network and then set to true
    },
    "apiVersion": "v1",
    "methods": [
        {
            "interface_method": "GetUserInfo",
            "plugin_method": "GetUserData" // Your function name - Expected args: [apikey: string]
        },
        {
            "interface_method": "CheckIfCached",
            "plugin_method": "GetTorrentCachedAvailability" // Expected args: [apikey: string, hashes: string[]]
        },
        {
            "interface_method": "GetCachedFiles",
            "plugin_method": "PlayCachedFile" // Expected args: [apikey: string, magnetlink: string, season?: int | null, episode?: int | null]
        },
        {
            "interface_method": "AddFilesToCache",
            "plugin_method": "AddFilesToCache" // Expected args: [apikey: string, magnetlink: string, season?: int | null, episode?: int | null]
        },
        {
            "interface_method": "GetTransferInfo",
            "plugin_method": "GetTransferInfo" // Expected args: [apikey: string, id: string]
        },
        {
            "interface_method": "CancelTransfer",
            "plugin_method": "CancelTransfer" // Expected args: [apikey: string, id: string]
        },
        {
            "interface_method": "UnrestrictLink",
            "plugin_method": "UnrestrictLink" // Expected args: [apikey: string, link: string]
        },
        {
            "interface_method": "GetDownloadLink",
            "plugin_method": "GetDownloadLink" // Expected args: [apikey: string, link: string]
        },
        {
            "interface_method": "GetTorrentList",
            "plugin_method": "GetTorrentList" // Expected args: [apikey: string]
        }
    ]
}
```
