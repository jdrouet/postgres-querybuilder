use crate::bucket::Bucket;
use crate::prelude::*;
use postgres_types::ToSql;

pub struct UpdateBuilder {
  table: String,
  fields: Vec<String>,
  conditions: Vec<String>,
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
      conditions: vec![],
      params: Bucket::new(),
    }
  }
}

impl UpdateBuilder {
  fn from_to_query(&self) -> String {
    format!("UPDATE {}", self.table)
  }

  fn set_to_query(&self) -> Option<String> {
    if self.fields.len() > 0 {
      let fields_query = self.fields.join(", ");
      Some(format!("SET {}", fields_query))
    } else {
      None
    }
  }

  fn where_to_query(&self) -> Option<String> {
    if self.conditions.len() > 0 {
      let where_query = self.conditions.join(" AND ");
      Some(format!("WHERE {}", where_query))
    } else {
      None
    }
  }
}

impl QueryBuilder for UpdateBuilder {
  fn add_param<T: 'static + ToSql + Sync + Clone>(&mut self, value: T) -> usize {
    self.params.push(value)
  }

  fn get_query(&self) -> String {
    let mut result: Vec<String> = vec![];
    result.push(self.from_to_query());
    match self.set_to_query() {
      Some(value) => result.push(value),
      None => (),
    };
    match self.where_to_query() {
      Some(value) => result.push(value),
      None => (),
    };
    result.join(" ")
  }

  fn get_ref_params(self) -> Vec<&'static (dyn ToSql + Sync)> {
    self.params.get_refs()
  }
}

impl QueryBuilderWithWhere for UpdateBuilder {
  fn where_condition(&mut self, raw: &str) {
    self.conditions.push(raw.to_string());
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
