#[macro_use]
extern crate log;

use actix_web::{guard, middleware, web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema, ID};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use model::Coder;
use sqlx::postgres::PgPool;
use std::env;

mod model;

async fn ping(_req: HttpRequest) -> impl Responder {
    format!(
        "I am healthy: {} v{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    )
}

struct Query;

#[Object(extends)]
impl Query {
    async fn coders<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Coder>> {
        let pool = ctx.data::<PgPool>().unwrap();
        let rows = Coder::read_all(&pool).await?;
        Ok(rows)
    }

    async fn coder<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Coder> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = Coder::read_one(&pool, &id).await?;
        Ok(row)
    }

    #[graphql(entity)]
    async fn find_coder_by_id<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Coder> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = Coder::read_one(&pool, &id).await?;
        Ok(row)
    }
}

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_coder(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: String,
    ) -> FieldResult<Coder> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = Coder::create(&pool, &title, &description).await?;
        Ok(row)
    }

    async fn delete_coder(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        Coder::delete(&pool, &id).await?;
        Ok(true)
    }

    async fn update_coder(
        &self,
        ctx: &Context<'_>,
        id: ID,
        description: String,
    ) -> FieldResult<Coder> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        let row = Coder::update(&pool, &id, &description).await?;
        Ok(row)
    }
}

type ServiceSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ServiceSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host = env::var("HOST").expect("HOST is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let db_pool = PgPool::connect(&database_url).await?;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db_pool)
        .finish();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .route("/ping", web::get().to(ping))
    });

    info!("Starting server");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
