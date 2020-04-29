use crate::bucket::Bucket;
use crate::prelude::*;
use postgres_types::ToSql;

pub struct UpdateBuilder {
  table: String,
  fields: Vec<String>,
  where_cols: Vec<String>,
  params: Bucket,
}

impl UpdateBuilder {
  /// Create a new update builder for a given table
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::UpdateBuilder;
  /// use postgres_querybuilder::prelude::{QueryBuilder, QueryBuilderWithSet, QueryBuilderWithWhere};
  ///
  /// let user_password = "password".to_string();
  /// let mut builder = UpdateBuilder::new("users");
  /// builder.set("username", "rick".to_string());
  /// builder.where_eq("id", 42);
  ///
  /// assert_eq!(builder.get_query(), "UPDATE users SET username = $1 WHERE id = $2");
  /// ```
  pub fn new(from: &str) -> Self {
    UpdateBuilder {
      table: from.into(),
      fields: vec![],
      where_cols: vec![],
      params: Bucket::new(),
    }
  }
}

impl QueryBuilder for UpdateBuilder {
  fn get_query(&self) -> String {
    let mut result = format!("UPDATE {}", self.table);
    if self.fields.len() > 0 {
      let fields_query = self.fields.join(", ");
      result = format!("{} SET {}", result, fields_query);
    }
    if self.where_cols.len() > 0 {
      let where_query = self.where_cols.join(" AND ");
      result = format!("{} WHERE {}", result, where_query);
    }
    result
  }

  fn get_ref_params(self) -> Vec<&'static (dyn ToSql + Sync)> {
    self.params.get_refs()
  }
}

impl QueryBuilderWithWhere for UpdateBuilder {
  fn where_eq<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T) {
    let index = self.params.push(value);
    self.where_cols.push(format!("{} = ${}", field, index));
  }

  fn where_ne<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T) {
    let index = self.params.push(value);
    self.where_cols.push(format!("{} <> ${}", field, index));
  }
}

impl QueryBuilderWithSet for UpdateBuilder {
  fn set<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T) {
    let index = self.params.push(value);
    self.fields.push(format!("{} = ${}", field, index));
  }

  fn set_computed(&mut self, field: &str, value: &str) {
    self.fields.push(format!("{} = {}", field, value));
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn from_scratch() {
    let builder = UpdateBuilder::new("publishers");
    assert_eq!(builder.get_query(), "UPDATE publishers");
  }

  #[test]
  fn with_fields_and_where() {
    let mut builder = UpdateBuilder::new("publishers");
    builder.where_eq("trololo", 42);
    builder.set("id", 5);
    assert_eq!(
      builder.get_query(),
      "UPDATE publishers SET id = $2 WHERE trololo = $1"
    );
  }

  #[test]
  fn with_computed_fields_and_where() {
    let mut builder = UpdateBuilder::new("publishers");
    builder.where_eq("trololo", 42);
    builder.set("id", 5);
    builder.set_computed("trololo", "md5(42)");
    assert_eq!(
      builder.get_query(),
      "UPDATE publishers SET id = $2, trololo = md5(42) WHERE trololo = $1"
    );
  }
}
