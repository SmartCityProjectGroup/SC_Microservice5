use moon::*;
use moon::config::CONFIG;
use crate::actix_cors::Cors;
use crate::actix_http::StatusCode;
use crate::actix_web::{App, web};
use crate::actix_web::middleware::{Compat, Condition, ErrorHandlers, Logger};

use crate::connect::{establish_connection, set_server_api_routes};

#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod connect;
mod actions;
mod api;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Tierheim")
        .default_styles(false)
        .append_to_head(r#"<link href="/_api/public/css/currentStyle.css" rel="stylesheet"/>"#)
        .body_content(r#"<div id="main"></div>"#)
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {


    let connection = establish_connection();

    let app = move || {
        let redirect = Redirect::new()
            .http_to_https(CONFIG.https)
            .port(CONFIG.redirect.port, CONFIG.port);

        App::new()
            .wrap(Condition::new(CONFIG.redirect.enabled, Compat::new(redirect)))
            .wrap(Logger::new("%r %s %D ms %a"))
            .wrap(Cors::default().allowed_origin_fn(move |origin, _| {
                if CONFIG.cors.origins.contains("*") {
                    return true;
                }
                let origin = match origin.to_str() {
                    Ok(origin) => origin,
                    Err(_) => return false,
                };
                CONFIG.cors.origins.contains(origin)
            }))

            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, error_handler::internal_server_error)
                .handler(StatusCode::NOT_FOUND, error_handler::not_found))

            .app_data(web::Data::new(connection.clone()))
    };

    start_with_app(frontend, up_msg_handler, app, set_server_api_routes).await;
    Ok(())

}
