# Developer guide

## Setting Chirpstack with different underlying databases

Commands need to be run within the developer environment, either nix directly or within docker.

### postgres

Setting up the DB and applying schema changes:

```
/root/.cargo/bin/diesel --config-file diesel_postgres.toml setup
/root/.cargo/bin/diesel --config-file diesel_postgres.toml migration run
/root/.cargo/bin/diesel --config-file diesel_postgres.toml print-schema > src/storage/schema_postgres.rs
```

Building and testing:

```
cargo build
cargo test
```

### sqlite

Setting up the DB and applying schema changes:

```
/root/.cargo/bin/diesel --config-file diesel_sqlite.toml --database-url sqlite:///tmp/chirp.sqlite setup --migration-dir migrations_sqlite/
/root/.cargo/bin/diesel --config-file diesel_sqlite.toml --database-url sqlite:///tmp/chirp.sqlite migration --migration-dir migrations_sqlite/ run
/root/.cargo/bin/diesel --config-file diesel_sqlite.toml --database-url sqlite:///tmp/chirp.sqlite print-schema > src/storage/schema_sqlite.rs
```

Building and testing:

```
cargo build --no-default-features --features sqlite --target-dir target-sqlite

TEST_POSTGRESQL_DSN=sqlite:///tmp/test.sqlite cargo test --no-default-features --features sqlite --target-dir target-sqlite
```

## Keep in mind differences between postgres/sqlite

### Type equivalence in Chirpstack

|postgres|sqlite|
|-|-|
|uuid|text|
|bytea|blob|
|json|text (using serde)|

### SQL differences

- Sqlite column are not strongly typed so any type can fit inside depending on [column affinity](https://www.sqlite.org/datatype3.html)
- However, diesel cli still uses it to guess the type of the generated schema_sqlite.rs
- Primary keys shall usually be marked as NOT NULL (see [section "3.5. The PRIMARY KEY"](https://www.sqlite.org/lang_createtable.html))
- Sqlite doesn't support array of values as opposed to Postgres' `int[]` for example
- When altering a table, a single operation can be done on one query with sqlite. Meaning there need to be one alter table per altered column (dropped, added or other) for example.
- Index on a table needs to be dropped before its target column. Because SQLite fails to drop the column if an index exists on it.
- Sqlite requires a complex procedure for some column operations like changing column type or removing the default of the column. ([see "simple procedure" in the second half of this section](https://www.sqlite.org/lang_altertable.html#otheralter)). An alternative could be creating a new column, moving the data in it, dropping the old and renaming the new to old if necessary.

### Diesel differences

- `for_update` to lock the DB for writing is not available with sqlite. This isn't an issue since there is a single Chirpstack instance per DB in this case.
- Timestamp with timezone needs to be "patched" from the diesel schema_sqlite.rs output, changing all Timestamp to TimestamptzSqlite. Through `chirpstack/src/storage/schema_sqlite.patch`.
