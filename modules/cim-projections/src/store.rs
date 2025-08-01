use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{ProjectionError, Result};

/// Store for projection read models
#[async_trait]
pub trait ProjectionStore<T>: Send + Sync
where
    T: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>,
{
    /// Save or update a read model
    async fn save(&self, id: &str, model: T) -> Result<()>;
    
    /// Get a read model by ID
    async fn get(&self, id: &str) -> Result<Option<T>>;
    
    /// Delete a read model
    async fn delete(&self, id: &str) -> Result<()>;
    
    /// List all read models
    async fn list(&self) -> Result<Vec<T>>;
    
    /// Query read models with a predicate
    async fn query<F>(&self, predicate: F) -> Result<Vec<T>>
    where
        F: Fn(&T) -> bool + Send + Sync;
    
    /// Clear all read models
    async fn clear(&self) -> Result<()>;
    
    /// Get the current position for a projection
    async fn get_position(&self, projection_name: &str) -> Result<u64>;
    
    /// Set the position for a projection
    async fn set_position(&self, projection_name: &str, position: u64) -> Result<()>;
}

/// In-memory projection store for testing and development
#[derive(Clone)]
pub struct InMemoryProjectionStore<T> {
    data: Arc<RwLock<HashMap<String, T>>>,
    positions: Arc<RwLock<HashMap<String, u64>>>,
}

impl<T> InMemoryProjectionStore<T>
where
    T: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            positions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<T> Default for InMemoryProjectionStore<T>
where
    T: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<T> ProjectionStore<T> for InMemoryProjectionStore<T>
where
    T: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static,
{
    async fn save(&self, id: &str, model: T) -> Result<()> {
        let mut data = self.data.write().await;
        data.insert(id.to_string(), model);
        Ok(())
    }
    
    async fn get(&self, id: &str) -> Result<Option<T>> {
        let data = self.data.read().await;
        Ok(data.get(id).cloned())
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        let mut data = self.data.write().await;
        data.remove(id);
        Ok(())
    }
    
    async fn list(&self) -> Result<Vec<T>> {
        let data = self.data.read().await;
        Ok(data.values().cloned().collect())
    }
    
    async fn query<F>(&self, predicate: F) -> Result<Vec<T>>
    where
        F: Fn(&T) -> bool + Send + Sync,
    {
        let data = self.data.read().await;
        Ok(data.values().filter(|v| predicate(v)).cloned().collect())
    }
    
    async fn clear(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.clear();
        Ok(())
    }
    
    async fn get_position(&self, projection_name: &str) -> Result<u64> {
        let positions = self.positions.read().await;
        Ok(positions.get(projection_name).copied().unwrap_or(0))
    }
    
    async fn set_position(&self, projection_name: &str, position: u64) -> Result<()> {
        let mut positions = self.positions.write().await;
        positions.insert(projection_name.to_string(), position);
        Ok(())
    }
}

/// PostgreSQL-backed projection store
#[cfg(feature = "postgres")]
pub struct PostgresProjectionStore<T> {
    pool: sqlx::PgPool,
    table_name: String,
    _phantom: std::marker::PhantomData<T>,
}

#[cfg(feature = "postgres")]
impl<T> PostgresProjectionStore<T>
where
    T: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>,
{
    pub async fn new(database_url: &str, table_name: &str) -> Result<Self> {
        let pool = sqlx::PgPool::connect(database_url)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        // Create tables if they don't exist
        let create_table = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                data JSONB NOT NULL,
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            
            CREATE TABLE IF NOT EXISTS projection_positions (
                projection_name TEXT PRIMARY KEY,
                position BIGINT NOT NULL,
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            "#,
            table_name
        );
        
        sqlx::query(&create_table)
            .execute(&pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        Ok(Self {
            pool,
            table_name: table_name.to_string(),
            _phantom: std::marker::PhantomData,
        })
    }
}

#[cfg(feature = "postgres")]
#[async_trait]
impl<T> ProjectionStore<T> for PostgresProjectionStore<T>
where
    T: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static,
{
    async fn save(&self, id: &str, model: T) -> Result<()> {
        let data = serde_json::to_value(&model)?;
        
        let query = format!(
            r#"
            INSERT INTO {} (id, data, updated_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (id) DO UPDATE
            SET data = $2, updated_at = NOW()
            "#,
            self.table_name
        );
        
        sqlx::query(&query)
            .bind(id)
            .bind(data)
            .execute(&self.pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get(&self, id: &str) -> Result<Option<T>> {
        let query = format!("SELECT data FROM {} WHERE id = $1", self.table_name);
        
        let row: Option<(serde_json::Value,)> = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        match row {
            Some((data,)) => {
                let model = serde_json::from_value(data)?;
                Ok(Some(model))
            }
            None => Ok(None),
        }
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        let query = format!("DELETE FROM {} WHERE id = $1", self.table_name);
        
        sqlx::query(&query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    async fn list(&self) -> Result<Vec<T>> {
        let query = format!("SELECT data FROM {} ORDER BY updated_at DESC", self.table_name);
        
        let rows: Vec<(serde_json::Value,)> = sqlx::query_as(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        let mut results = Vec::new();
        for (data,) in rows {
            let model = serde_json::from_value(data)?;
            results.push(model);
        }
        
        Ok(results)
    }
    
    async fn query<F>(&self, predicate: F) -> Result<Vec<T>>
    where
        F: Fn(&T) -> bool + Send + Sync,
    {
        // For PostgreSQL, we fetch all and filter in memory
        // Could be optimized with JSONB queries for specific cases
        let all = self.list().await?;
        Ok(all.into_iter().filter(|m| predicate(m)).collect())
    }
    
    async fn clear(&self) -> Result<()> {
        let query = format!("TRUNCATE TABLE {}", self.table_name);
        
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_position(&self, projection_name: &str) -> Result<u64> {
        let query = "SELECT position FROM projection_positions WHERE projection_name = $1";
        
        let row: Option<(i64,)> = sqlx::query_as(query)
            .bind(projection_name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        Ok(row.map(|(p,)| p as u64).unwrap_or(0))
    }
    
    async fn set_position(&self, projection_name: &str, position: u64) -> Result<()> {
        let query = r#"
            INSERT INTO projection_positions (projection_name, position, updated_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (projection_name) DO UPDATE
            SET position = $2, updated_at = NOW()
        "#;
        
        sqlx::query(query)
            .bind(projection_name)
            .bind(position as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| ProjectionError::Database(e.to_string()))?;
        
        Ok(())
    }
}