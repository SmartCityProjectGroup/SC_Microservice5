
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use lapin::{message::DeliveryResult, options::*, publisher_confirm::Confirmation, types::FieldTable,
    BasicProperties, Connection, ConnectionProperties};
use tracing::info;


pub fn establish_connection() -> MysqlConnection {
    use diesel::*;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))

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

