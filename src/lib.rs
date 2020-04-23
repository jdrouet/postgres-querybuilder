//! # Postgres QueryBuilder
//!
//! `postgres-querybuilder` is a tool to help you build sql queries that you can then use with [rust-postgres](https://github.com/sfackler/rust-postgres)
// --snip--

pub mod prelude;
mod select_builder;
mod update_builder;

pub use select_builder::SelectBuilder;
pub use update_builder::UpdateBuilder;
