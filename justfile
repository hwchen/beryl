serve:
    watchexec -r -s SIGKILL 'cargo build && BERYL_DATABASE_URL=clickhouse://127.0.0.1:9000 BERYL_SCHEMA_FILEPATH=test/schema.json RUST_LOG=info ./target/debug/beryl'
