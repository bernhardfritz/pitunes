use crate::graphql_schema::{Context, Schema};
use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("https://localhost:8080/graphql"); // TODO: hardcoded
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html) // TODO: html needs to be adapted to allow entering of api_key to be used for requests
}

#[post("/graphql")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let bla = web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(bla))
}
