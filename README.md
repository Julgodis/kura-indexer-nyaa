# くら - Nyaa Indexer

A lightweight and efficient indexer for Nyaa sites, designed to integrate seamlessly with the くら ecosystem.

## Configuration

Below is an example configuration file in TOML format:

```toml
[kura]
listen_addr = "127.0.0.1:19300"
db_path = "nyaa-indexer.db"

[nyaa]
url = "https://nyaa.si/"
update_interval = "15m"
requests_per_second = 5

[frontend]
```
