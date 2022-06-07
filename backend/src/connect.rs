use moon::*;
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::ConnectionManager;

use lapin::{message::DeliveryResult, options::*, publisher_confirm::Confirmation, types::FieldTable,
    BasicProperties, Connection, ConnectionProperties};
use tracing::info;
use crate::api::{add_new_pet, show_shelter_pets};
use crate::web;

pub type DBPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> DBPool {
    use diesel::*;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db_manager = ConnectionManager::<MysqlConnection>::new(database_url);

    diesel::r2d2::Pool::builder().build(db_manager)
        .unwrap()
}
/*
async fn connect_RabbitMQ() {
    let rabbitmq_url = "RABBITMQ_URL";
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);
    let connection = Connection::connect(rabbitmq_url, options).await.unwrap();
    let channel = connection.create_channel().await.unwrap();
}
*/

pub fn set_server_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/addpet", web::post().to(add_new_pet))
        .route("/showShelterPets", web::get().to(show_shelter_pets)
    );
}
