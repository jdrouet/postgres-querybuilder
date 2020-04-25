//! # Postgres QueryBuilder
//!
//! `postgres-querybuilder` is a tool to help you build sql queries that you can then use with [rust-postgres](https://github.com/sfackler/rust-postgres)
// --snip--

#[cfg(test)]
#[macro_use]
extern crate serial_test;

pub mod prelude;
mod select_builder;
mod update_builder;

pub use select_builder::SelectBuilder;
pub use update_builder::UpdateBuilder;

#[cfg(test)]
mod test {
  use super::prelude::*;
  use super::select_builder::SelectBuilder;
  use super::*;
  use postgres::{Client, Error, NoTls};
  use std::env;

  fn get_url() -> String {
    match env::var("DATABASE_URL") {
      Ok(value) => value,
      Err(_) => "postgres://postgres:password@localhost/postgres".into(),
    }
  }

  fn prepare(client: &mut Client) {
    client.execute("DROP TABLE IF EXISTS users;", &[]).unwrap();
    client
      .execute(
        "CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT UNIQUE NOT NULL);",
        &[],
      )
      .unwrap();
  }

  fn get_connection() -> Client {
    let url = get_url();
    let mut client = Client::connect(url.as_str(), NoTls).unwrap();
    prepare(&mut client);
    client
  }

  fn execute<T: QueryBuilder>(builder: T) -> Result<u64, Error> {
    let mut client = get_connection();
    let stmt = builder.get_query();
    let params = builder.get_ref_params();
    client.execute(stmt.as_str(), &params)
  }

  #[serial]
  #[test]
  fn select_limit_offset() {
    let mut builder = SelectBuilder::new("users");
    builder.select("id");
    builder.select("name");
    builder.limit(3);
    builder.offset(1);
    execute(builder).unwrap();
  }

  #[serial]
  #[test]
  fn select_where() {
    let mut builder = SelectBuilder::new("users");
    builder.select("id");
    builder.select("name");
    builder.where_eq("id", 42);
    execute(builder).unwrap();
  }
}
