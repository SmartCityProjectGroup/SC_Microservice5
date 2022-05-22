use moon::*;

#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod connect;
mod show_posts;
use crate::show_posts::*;

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


    show_posts();

    start(frontend, up_msg_handler, |_| {}).await


}
