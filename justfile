serve:
    watchexec -r -s SIGKILL 'cargo build && BERYL_SCHEMA_FILEPATH=test-schema/schema.json RUST_LOG=info ./target/debug/beryl'
