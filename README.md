# meili-cli

A CLI for Meilisearch

## install

```bash
cargo install --path .
```

## Usage

get basic help:

```bash
meili-cli --help
```

by default, meili-cli uses `localhost:7700`, you can change the address by specifying it as a first argument:

```bash
meili-cli my-meili-instance.com index create movies
```

### Creating an index

```bash
meili-cli index create index_name
```

### Adding documents

```bash
echo '{"id": 1, "foo": "bar}' | meili-cli documents -i index_name --update add
```

or

```bash
meili-cli documents -i index_name add --update documents.json
'''
### Supported operations

- [x] create, delete, list indexes
- [x] perform searches
- [x] add, remove documents
- [x] list, reset, modify settings
- [x] support for API keys
