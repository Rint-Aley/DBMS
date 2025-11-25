mod database;
mod grpc_server;
mod protobuf_database_conversations;

use grpc_server::table_api;
use std::error::Error;
use std::path::PathBuf;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_directory = match std::env::var("DB_DIRECTORY") {
        Ok(path) => PathBuf::from(path),
        Err(_) => PathBuf::from("database"),
    };

    let addr = "[::1]:50051".parse()?;
    let table_service = match grpc_server::MyTableService::new(database_directory) {
        Ok(res) => res,
        Err(error_details) => {
            eprintln!("Failed to set up database directory: {error_details}");
            std::process::exit(1);
        }
    };

    Server::builder()
        .add_service(table_api::table_service_server::TableServiceServer::new(
            table_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
