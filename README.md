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

## User docs

Beryl generally tries to serve directly from the table, and then just matching its endpoint's interface to a table's columns. For exmple:

```json
    {
      "name": "stocks",
      "primary": "company_id",
      "sql_select": { "table": { "name": "stocks" } },

      "interface": {
        "company_id": {
          "column": "company_id",
          "visible": true
        },
        "symbol": {
          "column": "ticker_symbol",
          "filter_type": "string_match",
          "visible": true,
          "is_text": true
        },
    ...
```

`stocks` is the name of the endpoint. `primary` is the primary key of the table, used in case you want to access the endpoint `/stocks/<company_id>` instead of by filtering. The `sql_select` here points to the table with the data.

For the interface, each field is either
- a query param to filter on
- a field returned in the response
- both

`company_id` can be filtered on by, `/stocks?company_id=12345`.

`column` is the column in the table referenced in `sql_select`, `stocks`.

When a field is `visible`, it will come back in the response. If `visible` is false, it will be able to filter on it, but will not show up in the response.

`is_text` is necessary to let beryl know whether to put single quotes around a value when passing it into the sql query. So for `symbol`, `symbol=AAPL` in the url query param would become `ticker_symbol = 'AAPL'` in the sql.

`filter_type` is optional, and will default to `compare`.
- `compare` allows the format `gt.100` as `greater than 100`. gt, lt, eq, gte, lte, neq are supported.
- `exact_match` is self explanatory.
- `string_match` allows a case-insensitive substring match. `LIKE '%str%'` in sqlspeak.
- `in_array`, if the col is of type array, will check if the value passed is in that array.

## Templates

beryl also supports templates, which allows for a sql select statement to replace a reference to the table. So, basically a materialized view.

The template supports the use of placeholder values. These values must be referenced from the endpoint interface.

```json
    {
      "name": "managers",
      "primary": "",
      "sql_select": { "template": { "template_path": "managers.sql" } },

      "interface": {
        "manager_id": {
          "column": "manager_id",
          "filter_type": "exact_match",
          "visible": true,
          "is_text": false,
          "is_template_var": true
        },
```

```sql
SELECT * from managers_table where company_id = {{manager_id}};
```

here, there's a template which references `manager_id`. The managers endpoint uses this template, so the interface must have a field in the interface whose name matches `manager_id`, and carries a field `is_template_var: true`.

## Environment

make sure env vars are set. check systemd and justfile for examples.
```
BERYL_TEMPLATES_PATH
BERYL_DATABASE_URL
BERYL_SCHEMA_FILEPATH
```
