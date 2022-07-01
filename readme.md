# muxa template

this is a template for using [axum](https://docs.rs/axum) and [muxa](https://github.com/annieversary/muxa)

## getting started

copy `.env.template` into `.env`

then run:
```bash
# skippable if you have sqlx installed
cargo install sqlx

# create the db
sqlx database create

# run
cargo r
```

maybe something else im forgetting, idk, you can probably figure it out
