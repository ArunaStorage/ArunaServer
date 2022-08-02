use std::sync::Arc;

use tonic::transport::Server;

use crate::{
    api::aruna::api::storage::services::v1::collection_service_server::CollectionServiceServer,
    database::connection::Database,
};

use super::services::collection::CollectionServiceImpl;

pub struct ServiceServer {}

impl ServiceServer {
    pub async fn run(&self) {
        let db = Database::new();
        let db_ref = Arc::new(db);

        let addr = "[::1]:50051".parse().unwrap();
        let collection_service = CollectionServiceImpl::new(db_ref).await;
        let collection_server = CollectionServiceServer::new(collection_service);
        println!("GreeterServer listening on {}", addr);

        Server::builder()
            .add_service(collection_server)
            .serve(addr)
            .await
            .unwrap();
    }
}
