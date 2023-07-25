use crate::middlelayer::create_request_types::CreateRequest;
use crate::middlelayer::db_handler::DatabaseHandler;
use crate::middlelayer::delete_request_types::DeleteRequest;
use crate::middlelayer::update_request_types::{
    DataClassUpdate, DescriptionUpdate, KeyValueUpdate, NameUpdate,
};
use crate::utils::conversions::get_token_from_md;
use aruna_cache::notifications::NotificationCache;
use aruna_cache::structs::Resource;
use aruna_policy::ape::policy_evaluator::PolicyEvaluator;
use aruna_policy::ape::structs::{
    ApeResourcePermission, Context, PermissionLevels, ResourceContext,
};
use aruna_rust_api::api::storage::models::v2::generic_resource;
use aruna_rust_api::api::storage::services::v2::dataset_service_server::DatasetService;
use aruna_rust_api::api::storage::services::v2::{
    CreateDatasetRequest, CreateDatasetResponse, DeleteDatasetRequest, DeleteDatasetResponse,
    GetDatasetRequest, GetDatasetResponse, GetDatasetsRequest, GetDatasetsResponse,
    SnapshotDatasetRequest, SnapshotDatasetResponse, UpdateDatasetDataClassRequest,
    UpdateDatasetDataClassResponse, UpdateDatasetDescriptionRequest,
    UpdateDatasetDescriptionResponse, UpdateDatasetKeyValuesRequest,
    UpdateDatasetKeyValuesResponse, UpdateDatasetNameRequest, UpdateDatasetNameResponse,
};
use diesel_ulid::DieselUlid;
use std::str::FromStr;
use std::sync::Arc;
use tonic::{Request, Response, Result};

crate::impl_grpc_server!(DatasetServiceImpl);

#[tonic::async_trait]
impl DatasetService for DatasetServiceImpl {
    async fn create_dataset(
        &self,
        request: Request<CreateDatasetRequest>,
    ) -> Result<Response<CreateDatasetResponse>> {
        log_received!(request);

        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error."
        );

        let request = CreateRequest::Dataset(request.into_inner());
        let parent = request
            .get_parent()
            .ok_or(tonic::Status::invalid_argument("Parent missing."))?;

        let ctx = Context::ResourceContext(ResourceContext::Dataset(ApeResourcePermission {
            id: tonic_invalid!(parent.get_id(), "Invalid parent id."),
            level: PermissionLevels::WRITE, // append?
            allow_sa: true,
        }));

        let user_id = tonic_auth!(
            &self.authorizer.check_context(&token, ctx).await,
            "Unauthorized."
        )
        .ok_or(tonic::Status::invalid_argument("User id missing."))?;

        let dataset = match tonic_internal!(
            self.database_handler
                .create_resource(request, user_id)
                .await,
            "Internal database error."
        ) {
            generic_resource::Resource::Dataset(d) => Some(d),
            _ => return Err(tonic::Status::unknown("This should not happen.")),
        };

        Ok(tonic::Response::new(CreateDatasetResponse { dataset }))
    }
    async fn get_dataset(
        &self,
        request: Request<GetDatasetRequest>,
    ) -> Result<Response<GetDatasetResponse>> {
        log_received!(request);

        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error."
        );

        let request = request.into_inner();
        let id = tonic_invalid!(
            DieselUlid::from_str(&request.dataset_id),
            "Invalid dataset id."
        );
        let ctx = Context::ResourceContext(ResourceContext::Dataset(ApeResourcePermission {
            id,
            level: PermissionLevels::READ, // append?
            allow_sa: true,
        }));

        let dataset = match tonic_internal!(
            self.cache
                .cache
                .get_resource(&Resource::Dataset(id))
                .ok_or(tonic::Status::not_found("Collection not found.")),
            "Internal database error."
        ) {
            generic_resource::Resource::Dataset(d) => Some(d),
            _ => return Err(tonic::Status::unknown("This should not happen.")),
        };
        Ok(tonic::Response::new(GetDatasetResponse { dataset }))
    }

    async fn update_dataset_name(
        &self,
        request: Request<UpdateDatasetNameRequest>,
    ) -> Result<Response<UpdateDatasetNameResponse>> {
        log_received!(request);

        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error."
        );

        let request = NameUpdate::Dataset(request.into_inner());
        let dataset_id = tonic_invalid!(request.get_id(), "Invalid dataset id.");
        let ctx = Context::ResourceContext(ResourceContext::Collection(ApeResourcePermission {
            id: dataset_id,
            level: PermissionLevels::WRITE, // append?
            allow_sa: true,
        }));

        let user_id = tonic_auth!(
            &self.authorizer.check_context(&token, ctx).await,
            "Unauthorized."
        )
        .ok_or(tonic::Status::invalid_argument("User id missing."))?;

        let dataset = match tonic_internal!(
            self.database_handler.update_name(request).await,
            "Internal database error."
        ) {
            generic_resource::Resource::Dataset(d) => Some(d),
            _ => return Err(tonic::Status::unknown("This should not happen.")),
        };

        Ok(tonic::Response::new(UpdateDatasetNameResponse { dataset }))
    }

    async fn update_dataset_description(
        &self,
        request: Request<UpdateDatasetDescriptionRequest>,
    ) -> Result<Response<UpdateDatasetDescriptionResponse>> {
        log_received!(request);

        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error."
        );

        let request = DescriptionUpdate::Dataset(request.into_inner());
        let dataset_id = tonic_invalid!(request.get_id(), "Invalid dataset id.");
        let ctx = Context::ResourceContext(ResourceContext::Collection(ApeResourcePermission {
            id: dataset_id,
            level: PermissionLevels::WRITE, // append?
            allow_sa: true,
        }));

        let user_id = tonic_auth!(
            &self.authorizer.check_context(&token, ctx).await,
            "Unauthorized."
        )
        .ok_or(tonic::Status::invalid_argument("User id missing."))?;

        let dataset = match tonic_internal!(
            self.database_handler.update_description(request).await,
            "Internal database error."
        ) {
            generic_resource::Resource::Dataset(d) => Some(d),
            _ => return Err(tonic::Status::unknown("This should not happen.")),
        };

        Ok(tonic::Response::new(UpdateDatasetDescriptionResponse {
            dataset,
        }))
    }

    async fn update_dataset_data_class(
        &self,
        request: Request<UpdateDatasetDataClassRequest>,
    ) -> Result<Response<UpdateDatasetDataClassResponse>> {
        log_received!(request);

        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error."
        );

        let request = DataClassUpdate::Dataset(request.into_inner());
        let dataset_id = tonic_invalid!(request.get_id(), "Invalid dataset id.");
        let ctx = Context::ResourceContext(ResourceContext::Collection(ApeResourcePermission {
            id: dataset_id,
            level: PermissionLevels::WRITE, // append?
            allow_sa: true,
        }));

        let user_id = tonic_auth!(
            &self.authorizer.check_context(&token, ctx).await,
            "Unauthorized."
        )
        .ok_or(tonic::Status::invalid_argument("User id missing."))?;

        let dataset = match tonic_internal!(
            self.database_handler.update_dataclass(request).await,
            "Internal database error."
        ) {
            generic_resource::Resource::Dataset(d) => Some(d),
            _ => return Err(tonic::Status::unknown("This should not happen.")),
        };

        Ok(tonic::Response::new(UpdateDatasetDataClassResponse {
            dataset,
        }))
    }
    async fn update_dataset_key_values(
        &self,
        request: Request<UpdateDatasetKeyValuesRequest>,
    ) -> Result<Response<UpdateDatasetKeyValuesResponse>> {
        log_received!(request);

        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error."
        );

        let request = KeyValueUpdate::Dataset(request.into_inner());
        let dataset_id = tonic_invalid!(request.get_id(), "Invalid dataset id.");
        let ctx = Context::ResourceContext(ResourceContext::Collection(ApeResourcePermission {
            id: dataset_id,
            level: PermissionLevels::WRITE,
            allow_sa: true,
        }));

        let user_id = tonic_auth!(
            &self.authorizer.check_context(&token, ctx).await,
            "Unauthorized."
        )
        .ok_or(tonic::Status::invalid_argument("User id missing."))?;

        let dataset = match tonic_internal!(
            self.database_handler.update_keyvals(request).await,
            "Internal database error."
        ) {
            generic_resource::Resource::Dataset(d) => Some(d),
            _ => return Err(tonic::Status::unknown("This should not happen.")),
        };

        Ok(tonic::Response::new(UpdateDatasetKeyValuesResponse {
            dataset,
        }))
    }

    async fn delete_dataset(
        &self,
        request: Request<DeleteDatasetRequest>,
    ) -> Result<Response<DeleteDatasetResponse>> {
        log_received!(request);

        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error."
        );

        let request = DeleteRequest::Dataset(request.into_inner());
        let id = tonic_invalid!(request.get_id(), "Invalid collection id.");
        let ctx = Context::ResourceContext(ResourceContext::Dataset(ApeResourcePermission {
            id,
            level: PermissionLevels::WRITE,
            allow_sa: true,
        }));

        tonic_auth!(
            &self.authorizer.check_context(&token, ctx).await,
            "Unauthorized."
        );

        tonic_internal!(
            self.database_handler.delete_resource(request).await,
            "Internal database error."
        );

        Ok(tonic::Response::new(DeleteDatasetResponse {}))
    }
    async fn get_datasets(
        &self,
        _request: Request<GetDatasetsRequest>,
    ) -> Result<Response<GetDatasetsResponse>> {
        todo!()
    }
    async fn snapshot_dataset(
        &self,
        _request: Request<SnapshotDatasetRequest>,
    ) -> Result<Response<SnapshotDatasetResponse>> {
        todo!()
    }
}
