use anyhow::Result;
use async_graphql::SimpleObject;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct Coder {
    pub id: sqlx::types::Uuid,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
}

impl Coder {
    pub async fn create(pool: &PgPool, title: &str, description: &str) -> Result<Coder> {
        let row = sqlx::query!(
            "INSERT INTO coder(name,description) VALUES ($1,$2) RETURNING id",
            title,
            description
        )
        .fetch_one(pool)
        .await?;

        Ok(Coder {
            id: row.id,
            name: title.to_string(),
            description: description.to_string(),
            created_at: sqlx::types::chrono::Utc::now(),
        })
    }

    pub async fn read_one(pool: &PgPool, id: &str) -> Result<Coder> {
        let row = sqlx::query_as!(
            Coder,
            "SELECT * FROM coder WHERE id = $1",
            Uuid::parse_str(id)?
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<Coder>> {
        let rows = sqlx::query_as!(Coder, "SELECT * FROM coder")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    pub async fn update(pool: &PgPool, id: &str, description: &str) -> Result<Coder> {
        sqlx::query!(
            "UPDATE coder SET description=$1 WHERE id = $2",
            description,
            Uuid::parse_str(id)?
        )
        .execute(pool)
        .await?;

        Ok(Coder::read_one(pool, id).await?)
    }

    pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM coder WHERE id = $1", Uuid::parse_str(id)?)
            .execute(pool)
            .await?;

        Ok(())
    }
}
