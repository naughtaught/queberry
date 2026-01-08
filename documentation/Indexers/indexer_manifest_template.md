# Example Indexer Manifest

```json
{
    "id": "com.yourname.indexer", // reverse domain notation
    "name": "Your Indexer Name",
    "filename": "your_indexer.wasm",
    "author": "Your Name",
    "version": "1.0.0", // semantic versioning
    "sources": ["torrents"], // torrents, usenet, etc..
    "types": ["indexer"],
    "cacheless": false,
    "permissions": {
        "network": ["https://api.your-indexer.com/*"] // Only network permissions permitted
        "allow_private_networks": false // Only needed if using a private network and then set to true
    },
    "apiVersion": "v1",
    "methods": [
        {
            "interface_method": "GetIndexerSources",
            "plugin_method": "get_sources" // Your function name - Expected args: [imdbId: string, season?: int | null, episode?: int | null]
        }
    ]
}
```
