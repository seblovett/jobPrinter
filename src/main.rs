
use std::{
    error::Error,
    net::Ipv4Addr,
};

use actix_web::{
    middleware::Logger,
    web::Data,
    App, HttpServer,
};
use utoipa::{
    OpenApi,
};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::todo::*;

mod todo;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    let store = Data::new(TodoStore::default());
        HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .service(utoipa_actix_web::scope("/api/todo").configure(todo::configure(store.clone())))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
