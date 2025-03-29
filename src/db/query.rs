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

    pub async fn fetch_all(mut self, pool: &DbPool) -> Result<Vec<T>> {
        let query = self.builder.build();
        let results = sqlx::query_as::<_, T>(query.sql())
            .fetch_all(pool)
            .await?;
        Ok(results)
    }
}