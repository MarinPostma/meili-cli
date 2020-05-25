# meili-cli

A CLI for Meilisearch

## Usage

get basic help:

```bash
meili-cli --help
```

by default, meili-cli uses `localhost:7700`, you can change the address by specifying it as a first argument:

```bash
meili-cli my-meili-instance.com index create movies
```

### Supported operations

- [x] create, delete, list indexes
- [x] perform searches
- [x] add, remove documents
- [x] list, reset, modify settings
- [x] support for API keys
