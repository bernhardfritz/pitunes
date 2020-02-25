use crate::graphiql::graphiql_source;
use crate::graphql_schema::{Context, Schema};
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use juniper::http::GraphQLRequest;
use std::sync::Arc;

#[get("/graphiql")]
async fn graphiql(app_state: web::Data<AppState>) -> HttpResponse {
    let html = graphiql_source(&format!("https://localhost:{}/graphql", app_state.port)[..]);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
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
