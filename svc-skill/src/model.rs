use anyhow::Result;
use async_graphql::{Context, FieldResult, Object, SimpleObject};
use chrono::prelude::*;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub struct Coder {
    pub id: sqlx::types::Uuid,
}

#[Object(extends)]
impl Coder {
    #[graphql(external)]
    async fn id(&self) -> &sqlx::types::Uuid {
        &self.id
    }

    async fn skills<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Skill>> {
        let pool = ctx.data::<PgPool>().unwrap();
        let rows = Skill::read_by_coder(&pool, &self.id).await?;
        Ok(rows)
    }
}

#[derive(SimpleObject, FromRow)]
pub struct Skill {
    pub id: sqlx::types::Uuid,
    title: String,
    description: String,
    coder_id: sqlx::types::Uuid,
    created_at: DateTime<Utc>,
}

impl Skill {
    pub async fn create(
        pool: &PgPool,
        coder_id: &str,
        title: &str,
        description: &str,
    ) -> Result<Skill> {
        let row = sqlx::query!(
            "INSERT INTO skill(title,description,coder_id) VALUES ($1,$2,$3) RETURNING id",
            title,
            description,
            Uuid::parse_str(coder_id)?
        )
        .fetch_one(pool)
        .await?;

        Ok(Skill {
            id: row.id,
            title: title.to_string(),
            description: description.to_string(),
            coder_id: Uuid::parse_str(coder_id)?,
            created_at: sqlx::types::chrono::Utc::now(),
        })
    }

    pub async fn read_one(pool: &PgPool, id: &str) -> Result<Skill> {
        let row = sqlx::query_as!(
            Skill,
            "SELECT * FROM skill WHERE id = $1",
            Uuid::parse_str(id)?
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn read_by_coder(pool: &PgPool, coder_id: &sqlx::types::Uuid) -> Result<Vec<Skill>> {
        let row = sqlx::query_as!(Skill, "SELECT * FROM skill WHERE coder_id = $1", coder_id)
            .fetch_all(pool)
            .await?;

        Ok(row)
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<Skill>> {
        let rows = sqlx::query_as!(Skill, "SELECT * FROM skill")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    pub async fn update(pool: &PgPool, id: &str, description: &str) -> Result<Skill> {
        sqlx::query!(
            "UPDATE skill SET description=$1 WHERE id = $2",
            description,
            Uuid::parse_str(id)?
        )
        .execute(pool)
        .await?;

        Ok(Skill::read_one(pool, id).await?)
    }

    pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM skill WHERE id = $1", Uuid::parse_str(id)?)
            .execute(pool)
            .await?;

        Ok(())
    }
}
