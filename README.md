# RxTrack

Set up instance using sqlite:

```sh
touch database.db
env DATABASE_URL="sqlite://./database.db" sea-orm-cli migrate refresh
```


Update entities (needs manual tweaks if using sqlite due to e.g. lack of date column type):

```sh
env DATABASE_URL="sqlite://./database.db" sea-orm-cli generate entity -o model/src/entities
```
