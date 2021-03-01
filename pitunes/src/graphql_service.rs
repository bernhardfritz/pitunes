use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use juniper::http::GraphQLRequest;

use crate::graphql_schema::{RequestContext, Schema};

#[post("/graphql")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<RequestContext>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let json = web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
