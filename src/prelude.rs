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
  fn has_params(&self) -> bool;
  fn next_index(&self) -> usize;
  fn get_query(&self) -> String;
  fn get_ref_params(self) -> Vec<&'static (dyn ToSql + Sync)>;
}

pub trait QueryBuilderWithWhere {
  fn where_eq<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T);
  fn where_ne<T: 'static + ToSql + Sync + Clone>(&mut self, field: &str, value: T);
}

pub trait QueryBuilderWithGroupBy {
  fn group_by(&mut self, field: &str);
}

pub trait QueryBuilderWithLimit {
  fn limit(&mut self, limit: i64);
}

pub trait QueryBuilderWithOffset {
  fn offset(&mut self, offset: u32);
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
