use crate::bucket::Bucket;
use crate::prelude::*;
use postgres_types::ToSql;

pub struct SelectBuilder {
    with_queries: Vec<(String, String)>,
    columns: Vec<String>,
    from_table: String,
    conditions: Vec<String>,
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
            with_queries: vec![],
            columns: vec![],
            from_table: from.into(),
            conditions: vec![],
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
    pub fn select(&mut self, column: &str) -> &mut Self {
        self.columns.push(column.to_string());
        self
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
    pub fn add_where_raw(&mut self, raw: String) -> &mut Self {
        self.conditions.push(raw);
        self
    }
}

impl SelectBuilder {
    fn with_queries_to_query(&self) -> Option<String> {
        if self.with_queries.len() > 0 {
            let result: Vec<String> = self
                .with_queries
                .iter()
                .map(|item| format!("{} AS ({})", item.0, item.1))
                .collect();
            Some(format!("WITH {}", result.join(", ")))
        } else {
            None
        }
    }

    fn select_to_query(&self) -> String {
        let columns = if self.columns.len() == 0 {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };
        format!("SELECT {}", columns)
    }

    fn from_to_query(&self) -> String {
        format!("FROM {}", self.from_table)
    }

    fn where_to_query(&self) -> Option<String> {
        if self.conditions.len() > 0 {
            let result = self.conditions.join(" AND ");
            Some(format!("WHERE {}", result))
        } else {
            None
        }
    }

    fn group_by_to_query(&self) -> Option<String> {
        if self.groups.len() > 0 {
            let result = self.groups.join(", ");
            Some(format!("GROUP BY {}", result))
        } else {
            None
        }
    }

    fn order_by_to_query(&self) -> Option<String> {
        if self.order.len() > 0 {
            let result: Vec<String> = self.order.iter().map(|order| order.to_string()).collect();
            Some(format!("ORDER BY {}", result.join(", ")))
        } else {
            None
        }
    }

    fn limit_to_query(&self) -> Option<String> {
        match self.limit.as_ref() {
            Some(limit) => Some(format!("LIMIT {}", limit)),
            None => None,
        }
    }

    fn offset_to_query(&self) -> Option<String> {
        match self.offset.as_ref() {
            Some(offset) => Some(format!("OFFSET {}", offset)),
            None => None,
        }
    }
}

impl QueryBuilder for SelectBuilder {
    fn add_param<T: 'static + ToSql + Sync + Clone>(&mut self, value: T) -> usize {
        self.params.push(value)
    }

    fn get_query(&self) -> String {
        let mut sections: Vec<String> = vec![];
        match self.with_queries_to_query() {
            Some(value) => sections.push(value),
            None => (),
        };
        sections.push(self.select_to_query());
        sections.push(self.from_to_query());
        match self.where_to_query() {
            Some(value) => sections.push(value),
            None => (),
        };
        match self.group_by_to_query() {
            Some(value) => sections.push(value),
            None => (),
        };
        match self.order_by_to_query() {
            Some(value) => sections.push(value),
            None => (),
        };
        match self.limit_to_query() {
            Some(value) => sections.push(value),
            None => (),
        };
        match self.offset_to_query() {
            Some(value) => sections.push(value),
            None => (),
        };
        sections.join(" ")
    }

    fn get_ref_params(self) -> Vec<&'static (dyn ToSql + Sync)> {
        self.params.get_refs()
    }
}

impl QueryBuilderWithWhere for SelectBuilder {
    fn where_condition(&mut self, raw: &str) -> &mut Self {
        self.conditions.push(raw.to_string());
        self
    }
}

impl QueryBuilderWithLimit for SelectBuilder {
    fn limit(&mut self, limit: i64) -> &mut Self {
        let index = self.params.push(limit);
        self.limit = Some(format!("${}", index));
        self
    }
}

impl QueryBuilderWithOffset for SelectBuilder {
    fn offset(&mut self, offset: i64) -> &mut Self {
        let index = self.params.push(offset);
        self.offset = Some(format!("${}", index));
        self
    }
}

impl QueryBuilderWithJoin for SelectBuilder {
    fn inner_join(&mut self, table_name: &str, relation: &str) -> &mut Self {
        self.joins
            .push(Join::Inner(table_name.to_string(), relation.to_string()));
        self
    }

    fn left_join(&mut self, table_name: &str, relation: &str) -> &mut Self {
        self.joins.push(Join::LeftOuter(
            table_name.to_string(),
            relation.to_string(),
        ));
        self
    }

    fn left_outer_join(&mut self, table_name: &str, relation: &str) -> &mut Self {
        self.joins
            .push(Join::Left(table_name.to_string(), relation.to_string()));
        self
    }
}

impl QueryBuilderWithGroupBy for SelectBuilder {
    fn group_by(&mut self, field: &str) -> &mut Self {
        self.groups.push(field.to_string());
        self
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

impl QueryBuilderWithQueries for SelectBuilder {
    fn with_query(&mut self, name: &str, query: &str) -> &mut Self {
        self.with_queries.push((name.into(), query.into()));
        self
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

    #[test]
    fn with_subquery() {
        let mut builder = SelectBuilder::new("publishers_view");
        builder.with_query(
            "publishers_count",
            "SELECT publisher_id, count(*) FROM articles GROUP BY publisher_id",
        );
        builder.with_query("publishers_subquery", "SELECT * FROM publishers");
        assert_eq!(
      builder.get_query(),
      "WITH publishers_count AS (SELECT publisher_id, count(*) FROM articles GROUP BY publisher_id), publishers_subquery AS (SELECT * FROM publishers) SELECT * FROM publishers_view"
    );
    }
}
