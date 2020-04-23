# Postgres QueryBuilder

`postgres-querybuilder` is a tool to help you write dynamic sql queries in rust and make them work with [rust-postgres](https://github.com/sfackler/rust-postgres).

## Example

```rust
let client = pool.get().await?;
let mut builder = SelectBuilder::new("users");
builder.select("id");
builder.select("email");
builder.where_eq("password", "123456".to_string());
let query = builder.get_query();
let params = builder.get_ref_params();
let stmt = client.prepare(query.as_str()).await?;
let rows = client.query(&stmt, &params).await?;
let user = rows.first().map(User::from);
```

## TODO

- [ ] Select query
  - [x] choose columns
  - [x] where equal
  - [x] where not equal
  - [ ] or where condition
  - [x] group by
  - [x] limit
  - [x] offset
- [ ] Update query
  - [x] set value
  - [x] where equal
  - [x] where not equal
  - [ ] or where condition
  - [ ] returning
- [ ] Insert query
- [ ] Delete query
- [ ] `WITH` query
- [ ] from subrequest
