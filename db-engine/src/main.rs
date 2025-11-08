mod database;
mod grpc_server;

use grpc_server::table_api;
use std::error::Error;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let database_directory = match std::env::var("DB_DIRECTORY") {
    //     Ok(path) => path,
    //     Err(_) => "database".to_string(),
    // };

    let addr = "[::1]:50051".parse()?;
    let table_service = grpc_server::MyTableService::default();

    Server::builder()
        .add_service(table_api::table_service_server::TableServiceServer::new(
            table_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
