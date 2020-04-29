use crate::bucket::Bucket;
use crate::prelude::*;
use postgres_types::ToSql;

pub struct SelectBuilder {
  select_cols: Vec<String>,
  from_table: String,
  where_cols: Vec<String>,
  joins: Vec<Join>,
  groups: Vec<String>,
  order: Vec<Order>,
  limit: Option<String>,
  offset: Option<String>,
  params: Bucket,
}

impl SelectBuilder {
  /// Create a new select query for a given table
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  ///
  /// let mut builder = SelectBuilder::new("users");
  /// ```
  pub fn new(from: &str) -> Self {
    SelectBuilder {
      select_cols: vec![],
      from_table: from.into(),
      where_cols: vec![],
      joins: vec![],
      groups: vec![],
      order: vec![],
      limit: None,
      offset: None,
      params: Bucket::new(),
    }
  }

  /// Add a column to select
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilder;
  ///
  /// let mut builder = SelectBuilder::new("users");
  /// builder.select("id");
  /// builder.select("email");
  ///
  /// assert_eq!(builder.get_query(), "SELECT id, email FROM users");
  /// ```
  pub fn select(&mut self, column: &str) {
    self.select_cols.push(column.to_string());
  }

  /// Add a raw where condition
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilder;
  ///
  /// let mut builder = SelectBuilder::new("users");
  /// builder.add_where_raw("something IS NULL".into());
  ///
  /// assert_eq!(builder.get_query(), "SELECT * FROM users WHERE something IS NULL");
  /// ```
  pub fn add_where_raw(&mut self, raw: String) {
    self.where_cols.push(raw);
  }

  /// Add a parameter to the list of parameters. This is mostly used internally.
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilder;
  ///
  /// let user_password = "password".to_string();
  /// let mut builder = SelectBuilder::new("users");
  /// let index = builder.add_param(user_password);
  /// builder.add_where_raw(format!("password = MD5(${})", index));
  ///
  /// assert_eq!(builder.get_query(), "SELECT * FROM users WHERE password = MD5($1)");
  /// ```
  pub fn add_param(&mut self, raw: String) -> usize {
    self.params.push(raw)
  }
}

impl QueryBuilder for SelectBuilder {
  fn get_query(&self) -> String {
    let columns = if self.select_cols.len() == 0 {
      "*".to_string()
    } else {
      self.select_cols.join(", ")
    };
    let mut result = format!("SELECT {} FROM {}", columns, self.from_table);
    for join in self.joins.iter() {
      result = format!("{} {}", result, join.to_string());
    }
    if self.where_cols.len() > 0 {
      let where_query = self.where_cols.join(" AND ");
      result = format!("{} WHERE {}", result, where_query);
    }
    if self.groups.len() > 0 {
      result = format!("{} GROUP BY {}", result, self.groups.join(", "));
    }
    if self.order.len() > 0 {
      let order: Vec<String> = self.order.iter().map(|order| order.to_string()).collect();
      result = format!("{} ORDER BY {}", result, order.join(", "));
    }
    if let Some(limit) = self.limit.as_ref() {
      result = format!("{} LIMIT {}", result, limit);
    }
    if let Some(offset) = self.offset.as_ref() {
      result = format!("{} OFFSET {}", result, offset);
    }
    result
  }

  fn get_ref_params(self) -> Vec<&'static (dyn ToSql + Sync)> {
    self.params.get_refs()
  }
}

impl QueryBuilderWithWhere for SelectBuilder {
  fn where_eq<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T) {
    let index = self.params.push(value);
    self.where_cols.push(format!("{} = ${}", field, index));
  }

  fn where_ne<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T) {
    let index = self.params.push(value);
    self.where_cols.push(format!("{} <> ${}", field, index));
  }
}

impl QueryBuilderWithLimit for SelectBuilder {
  fn limit(&mut self, limit: i64) {
    let index = self.params.push(limit);
    self.limit = Some(format!("${}", index));
  }
}

impl QueryBuilderWithOffset for SelectBuilder {
  fn offset(&mut self, offset: i64) {
    let index = self.params.push(offset);
    self.offset = Some(format!("${}", index));
  }
}

impl QueryBuilderWithJoin for SelectBuilder {
  fn inner_join(&mut self, table_name: &str, relation: &str) {
    self
      .joins
      .push(Join::Inner(table_name.to_string(), relation.to_string()));
  }

  fn left_join(&mut self, table_name: &str, relation: &str) {
    self.joins.push(Join::LeftOuter(
      table_name.to_string(),
      relation.to_string(),
    ));
  }

  fn left_outer_join(&mut self, table_name: &str, relation: &str) {
    self
      .joins
      .push(Join::Left(table_name.to_string(), relation.to_string()));
  }
}

impl QueryBuilderWithGroupBy for SelectBuilder {
  fn group_by(&mut self, field: &str) {
    self.groups.push(field.to_string());
  }
}

impl QueryBuilderWithOrder for SelectBuilder {
  /// Add order attribute to request
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  /// use postgres_querybuilder::prelude::Order;
  /// use postgres_querybuilder::prelude::QueryBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilderWithOrder;
  ///
  /// let mut builder = SelectBuilder::new("users");
  /// builder.order_by(Order::Asc("name".into()));
  ///
  /// assert_eq!(builder.get_query(), "SELECT * FROM users ORDER BY name ASC");
  /// ```
  fn order_by(&mut self, field: Order) {
    self.order.push(field);
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn from_scratch() {
    let builder = SelectBuilder::new("publishers");
    assert_eq!(builder.get_query(), "SELECT * FROM publishers");
  }

  #[test]
  fn with_columns() {
    let mut builder = SelectBuilder::new("publishers");
    builder.select("id");
    builder.select("name");
    assert_eq!(builder.get_query(), "SELECT id, name FROM publishers");
  }

  #[test]
  fn with_limit() {
    let mut builder = SelectBuilder::new("publishers");
    builder.select("id");
    builder.limit(10);
    assert_eq!(builder.get_query(), "SELECT id FROM publishers LIMIT $1");
  }

  #[test]
  fn with_limit_offset() {
    let mut builder = SelectBuilder::new("publishers");
    builder.select("id");
    builder.limit(10);
    builder.offset(5);
    assert_eq!(
      builder.get_query(),
      "SELECT id FROM publishers LIMIT $1 OFFSET $2"
    );
  }

  #[test]
  fn with_where_eq() {
    let mut builder = SelectBuilder::new("publishers");
    builder.select("id");
    builder.select("name");
    builder.where_eq("trololo", 42);
    builder.where_eq("tralala", true);
    builder.where_ne("trululu", "trololo");
    assert_eq!(
      builder.get_query(),
      "SELECT id, name FROM publishers WHERE trololo = $1 AND tralala = $2 AND trululu <> $3"
    );
  }

  #[test]
  fn with_order() {
    let mut builder = SelectBuilder::new("publishers");
    builder.select("id");
    builder.order_by(Order::Asc("id".into()));
    builder.order_by(Order::Desc("name".into()));
    assert_eq!(
      builder.get_query(),
      "SELECT id FROM publishers ORDER BY id ASC, name DESC"
    );
  }
}
