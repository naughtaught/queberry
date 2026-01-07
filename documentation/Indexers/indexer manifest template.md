```json
{
    "id": "com.yourname.indexer", // reverse domain notation
    "name": "Your Indexer Name",
    "filename": "your_indexer.wasm",
    "author": "Your Name",
    "version": "1.0.0", // semantic versioning
    "sources": ["torrents"], // torrents, usenet, etc..
    "types": ["indexer"],
    "permissions": {
        "network": ["https://api.your-indexer.com/*"] // Only network permissions permitted
    },
    "apiVersion": "v1",
    "methods": [
        {
            "interface_method": "GetIndexerSources",
            "plugin_method": "get_sources" // Your function name - Expected args: [imdbId, season?, episode?]
        }
    ]
}
```
