use sqlx::{FromRow, MySql, QueryBuilder, Execute};
use sqlx::mysql::MySqlRow;
use crate::db::pool::DbPool;
use anyhow::Result;

pub struct SafeQuery<'a, T> {
    builder: QueryBuilder<'a, MySql>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> SafeQuery<'a, T> 
where 
    T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin
{
    pub fn new() -> Self {
        Self {
            builder: QueryBuilder::new(String::new()),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn select(mut self, columns: &'a [&'a str]) -> Self {
        self.builder.push("SELECT ");
        let mut separated = self.builder.separated(", ");
        for col in columns {
            separated.push(*col);
        }
        self
    }

    pub fn from(mut self, table: &'a str) -> Self {
        self.builder.push(" FROM ").push(table);
        self
    }

    pub fn where_eq(mut self, column: &'a str, value: &'a str) -> Self {
        self.builder
            .push(" WHERE ")
            .push(column)
            .push(" = ")
            .push_bind(value);
        self
    }

    pub async fn fetch_all(mut self, pool: &DbPool) -> Result<Vec<T>> {
        let query = self.builder.build();
        let results = sqlx::query_as::<_, T>(query.sql())
            .fetch_all(pool)
            .await?;
        Ok(results)
    }

    pub async fn fetch_one(mut self, pool: &DbPool) -> Result<T> {
        let query = self.builder.build();
        let result = sqlx::query_as::<_, T>(query.sql())
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    pub async fn fetch_optional(mut self, pool: &DbPool) -> Result<Option<T>> {
        let query = self.builder.build();
        let result = sqlx::query_as::<_, T>(query.sql())
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }
}