# web_starter
web starter

## sql

```shell
cargo install sea-orm-cli
sea-orm-cli migrate up
sea-orm-cli migrate down
sea-orm-cli generate entity -s public --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --date-time-crate chrono -o ./web_starter/src/entity
```