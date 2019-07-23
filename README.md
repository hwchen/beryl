# Beryl

An API server for read-only databases with simple configuration and flexible filtering.

Supports clickhouse, with plans to support postgres and mysql.

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

#### sql templates
special feature! totally in alpha.

In the schema, for and endpoint you can specify a sql template instead of a table name. This template uses Tera to render, which uses a jinja2 inspired syntax.

The vars in the sql template must match the param name in the endpoint's interface. That param must also have the field `"is_template_var": true`, and the 'filter_type' must be set to `exact_match` or `string_match`. The sql itself must handle the quotes around a string type.

Then, when using that endpoint, those params which were specified as template vars must be used in the query params, otherwise the template will fail to render.
