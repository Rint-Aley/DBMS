use table_api::table_service_server::TableService;
use table_api::{
    AddRequest, ChangeRequest, DeleteRequest, Field, Filter, FilterOption, Order, OrderOption,
    PrimaryKey, RecordsInfo, SelectRequest, Table, TableList, Type, Value, ValueSequence, Void,
};
use tonic::{Request, Response, Status};

pub mod table_api {
    tonic::include_proto!("table_api");
}

#[derive(Debug, Default)]
pub struct MyTableService {}

#[tonic::async_trait]
impl TableService for MyTableService {
    async fn get_table_list(&self, request: Request<Void>) -> Result<Response<TableList>, Status> {
        unimplemented!()
    }

    async fn drop_table(&self, request: Request<Table>) -> Result<Response<Void>, Status> {
        unimplemented!()
    }
    async fn delete_table(&self, request: Request<Table>) -> Result<Response<Void>, Status> {
        unimplemented!()
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
        unimplemented!()
    }
    async fn add_records(&self, request: Request<AddRequest>) -> Result<Response<Void>, Status> {
        unimplemented!()
    }
    async fn delete_records(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<Void>, Status> {
        unimplemented!()
    }
    async fn change_records(
        &self,
        request: Request<ChangeRequest>,
    ) -> Result<Response<Void>, Status> {
        unimplemented!()
    }
}
