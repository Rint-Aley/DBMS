use crate::database::structures::{self, filters};

use super::database;
use std::path::PathBuf;
use table_api::table_service_server::TableService;
use table_api::{
    AddRequest, ChangeRequest, DeleteRequest, Field, Filter, FilterOption, RecordsInfo,
    SelectRequest, Table, TableList, TableMetadata, Type, Value, ValueSequence, Void,
};
use tonic::{Request, Response, Status};

pub mod table_api {
    tonic::include_proto!("table_api");
}

#[derive(Debug, Default)]
pub struct MyTableService {
    database_path: PathBuf,
}

impl MyTableService {
    pub fn new(database_path: PathBuf) -> Result<Self, String> {
        if database_path.exists() {
            if database_path.is_dir() {
                return Ok(MyTableService { database_path });
            } else {
                return Err(format!(
                    "Path exists but is not a directory: {}",
                    database_path.display()
                ));
            }
        }

        match std::fs::create_dir_all(&database_path) {
            Ok(()) => Ok(MyTableService { database_path }),
            Err(e) => Err(format!(
                "Failed to create directory {}: {}",
                database_path.display(),
                e
            )),
        }
    }
}

#[tonic::async_trait]
impl TableService for MyTableService {
    async fn get_table_list(&self, request: Request<Void>) -> Result<Response<TableList>, Status> {
        let table_names = match database::get_table_list(&self.database_path) {
            Ok(names) => names,
            Err(error_details) => {
                return Err(Status::new(tonic::Code::Aborted, error_details));
            }
        };
        let grpc_tables: Vec<Table> = table_names
            .into_iter()
            .map(|table_name| Table { name: table_name })
            .collect();
        let table_list = TableList {
            tables: grpc_tables,
        };
        Ok(Response::new(table_list))
    }

    async fn create_table(
        &self,
        request: Request<TableMetadata>,
    ) -> Result<Response<Void>, Status> {
        let table_info = request.into_inner();
        let table_name = table_info.name;
        let fields = table_info
            .fields
            .into_iter()
            .map(|proto_field| {
                let type_ = proto_field.r#type().into();
                structures::Field {
                    name: proto_field.name,
                    type_,
                    nullable: false,
                }
            })
            .collect();
        let pk = table_info.primary_key as u16;
        let indexes = table_info.indexes.into_iter().map(|el| el as u16).collect();
        let metadata = match structures::TableMetadata::new(fields, pk, indexes) {
            Ok(metadata) => metadata,
            Err(error_details) => {
                return Err(Status::new(tonic::Code::Aborted, error_details));
            }
        };
        match database::create_table(&self.database_path, &table_name, metadata) {
            Ok(()) => Ok(Response::new(Void {})),
            Err(error_details) => Err(Status::new(tonic::Code::Aborted, error_details)),
        }
    }

    async fn drop_table(&self, request: Request<Table>) -> Result<Response<Void>, Status> {
        let table = request.into_inner();
        let table_path = self.database_path.join(table.name);
        match database::clear_table(&table_path) {
            Ok(()) => Ok(Response::new(Void {})),
            Err(error_details) => Err(Status::new(tonic::Code::Aborted, error_details)),
        }
    }

    async fn delete_table(&self, request: Request<Table>) -> Result<Response<Void>, Status> {
        let table = request.into_inner();
        let table_path = self.database_path.join(table.name);
        match database::delete_table(&table_path) {
            Ok(()) => Ok(Response::new(Void {})),
            Err(err_info) => Err(Status::new(tonic::Code::Aborted, err_info)),
        }
    }

    async fn create_backup(&self, request: Request<Table>) -> Result<Response<Void>, Status> {
        unimplemented!()
    }

    async fn export(&self, request: Request<Table>) -> Result<Response<Void>, Status> {
        unimplemented!()
    }

    async fn select_records(
        &self,
        request: Request<SelectRequest>,
    ) -> Result<Response<RecordsInfo>, Status> {
        let request = request.into_inner();
        let table_name = request.table.unwrap().name;
        let table_path = self.database_path.join(table_name);
        let filters: Vec<database::structures::FilterOption> = request
            .filters
            .into_iter()
            .map(|filter_option| filter_option.try_into().unwrap())
            .collect();

        match database::get_records(&table_path, &filters) {
            Ok(data) => Ok(Response::new(data.into())),
            Err(error_details) => Err(Status::new(tonic::Code::Aborted, error_details)),
        }
    }

    async fn add_records(&self, request: Request<AddRequest>) -> Result<Response<Void>, Status> {
        let request = request.into_inner();
        let table_name = request.table.unwrap().name;
        let table_path = self.database_path.join(table_name);
        unimplemented!()
    }

    async fn delete_records(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<Void>, Status> {
        let request = request.into_inner();
        let table_name = request.table.unwrap().name;
        let table_path = self.database_path.join(table_name);
        let filters: Vec<database::structures::FilterOption> = request
            .filters
            .into_iter()
            .map(|filter_option| filter_option.try_into().unwrap())
            .collect();

        match database::delete_records(&table_path, &filters) {
            Ok(_) => Ok(Response::new(Void {})),
            Err(error_details) => Err(Status::new(tonic::Code::Aborted, error_details)),
        }
    }

    async fn change_records(
        &self,
        request: Request<ChangeRequest>,
    ) -> Result<Response<Void>, Status> {
        let request = request.into_inner();
        let table_name = request.table.unwrap().name;
        let table_path = self.database_path.join(table_name);
        unimplemented!()
    }
}
