use postgres_types::ToSql;

pub enum Join {
  Inner(String, String),
  Left(String, String),
  LeftOuter(String, String),
}

impl Join {
  pub fn to_string(&self) -> String {
    match self {
      Join::Inner(table, constraint) => format!("INNER JOIN {} ON {}", table, constraint),
      Join::Left(table, constraint) => format!("LEFT JOIN {} ON {}", table, constraint),
      Join::LeftOuter(table, constraint) => format!("LEFT OUTER JOIN {} ON {}", table, constraint),
    }
  }
}

pub trait QueryBuilder {
  fn add_param<T: 'static + ToSql + Sync + Clone>(&mut self, value: T) -> usize;
  fn get_query(&self) -> String;
  fn get_ref_params(self) -> Vec<&'static (dyn ToSql + Sync)>;
}

pub trait QueryBuilderWithWhere: QueryBuilder {
  /// Add where condition to query
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilderWithWhere;
  ///
  /// let user_password = "password".to_string();
  /// let mut builder = SelectBuilder::new("users");
  /// let first = builder.add_param(18);
  /// let second = builder.add_param(28);
  /// let condition = format!("age = ${} OR age = ${}", first, second);
  /// builder.where_condition(condition.as_str());
  ///
  /// assert_eq!(builder.get_query(), "SELECT * FROM users WHERE age = $1 OR age = $2");
  /// ```
  fn where_condition(&mut self, raw: &str);

  /// Add where equal condition to query
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilderWithWhere;
  ///
  /// let user_password = "password".to_string();
  /// let mut builder = SelectBuilder::new("users");
  /// builder.where_eq("id", 42);
  ///
  /// assert_eq!(builder.get_query(), "SELECT * FROM users WHERE id = $1");
  /// ```
  fn where_eq<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T) {
    let index = self.add_param(value);
    let condition = format!("{} = ${}", field, index);
    self.where_condition(condition.as_str());
  }

  /// Add where not equal condition to query
  ///
  /// # Examples
  ///
  /// ```
  /// use postgres_querybuilder::SelectBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilder;
  /// use postgres_querybuilder::prelude::QueryBuilderWithWhere;
  ///
  /// let user_password = "password".to_string();
  /// let mut builder = SelectBuilder::new("users");
  /// builder.where_ne("id", 42);
  ///
  /// assert_eq!(builder.get_query(), "SELECT * FROM users WHERE id <> $1");
  /// ```
  fn where_ne<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T) {
    let index = self.add_param(value);
    let condition = format!("{} <> ${}", field, index);
    self.where_condition(condition.as_str());
  }
}

pub trait QueryBuilderWithGroupBy {
  fn group_by(&mut self, field: &str);
}

pub trait QueryBuilderWithLimit {
  fn limit(&mut self, limit: i64);
}

pub trait QueryBuilderWithOffset {
  fn offset(&mut self, offset: i64);
}

pub trait QueryBuilderWithJoin {
  fn inner_join(&mut self, table_name: &str, relation: &str);
  fn left_join(&mut self, table_name: &str, relation: &str);
  fn left_outer_join(&mut self, table_name: &str, relation: &str);
}

pub trait QueryBuilderWithSet {
  fn set<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T);
  fn set_computed(&mut self, field: &str, value: &str);
}

pub enum Order {
  Asc(String),
  Desc(String),
}

impl Order {
  pub fn to_string(&self) -> String {
    match self {
      Order::Asc(column) => format!("{} ASC", column),
      Order::Desc(column) => format!("{} DESC", column),
    }
  }
}

pub trait QueryBuilderWithOrder {
  fn order_by(&mut self, field: Order);
}
