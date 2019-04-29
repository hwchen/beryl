# Beryl

## For Developers

### Dev Environment

Install [clickhouse](https://clickhouse.yandex/docs/en/getting_started/#installation)

To make life easier for developers, I've set up a simple dev environment using:
- [just](https://github.com/casey/just) (a command runner)
- [watchexec](https://github.com/watchexec/watchexec) (executes command on file changes)

You can install them via `cargo` or check their webpage:
```
cargo install just
cargo install watchexec
```

Make sure your `~/.cargo/bin` is in your `PATH`.

I also highly recommended using something like [direnv](https://github.com/direnv/direnv) to manage environment variables.

### Run tests

Create test backend (run once)
```
$ just create-test
```

Launch local test server (run this in separate terminal from next command, or as a background process)
```
$ just serve
```

Run tests against test server
```
$ just test
```
