use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, name: String, email: String) -> Result<User, sqlx::Error>;
    async fn update(
        &self,
        id: Uuid,
        name: String,
        email: String,
    ) -> Result<User, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create(&self, name: String, email: String) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(email)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        name: String,
        email: String,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET name = $1, email = $2, updated_at = NOW() WHERE id = $3 RETURNING *",
        )
        .bind(name)
        .bind(email)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
