use crate::auth::permission_handler::PermissionHandler;
use crate::auth::structs::Context;
use crate::caching::cache::Cache;
use crate::database::enums::DbPermissionLevel;
use crate::middlelayer::db_handler::DatabaseHandler;
use crate::utils::conversions::get_token_from_md;
use aruna_rust_api::api::storage::services::v2::authorization_service_server::AuthorizationService;
use aruna_rust_api::api::storage::services::v2::{
    CreateAuthorizationRequest, CreateAuthorizationResponse, DeleteAuthorizationRequest,
    DeleteAuthorizationResponse, GetAuthorizationsRequest, GetAuthorizationsResponse,
    UpdateAuthorizationsRequest, UpdateAuthorizationsResponse,
};
use diesel_ulid::DieselUlid;
use std::str::FromStr;
use std::sync::Arc;

crate::impl_grpc_server!(AuthorizationServiceImpl);

#[tonic::async_trait]
impl AuthorizationService for AuthorizationServiceImpl {
    /// CreateAuthorization
    ///
    /// Status: BETA
    ///
    /// This creates a user-specific attribute that handles permission for a
    /// specific resource
    async fn create_authorization(
        &self,
        request: tonic::Request<CreateAuthorizationRequest>,
    ) -> std::result::Result<tonic::Response<CreateAuthorizationResponse>, tonic::Status> {
        log_received!(&request);
        let token = tonic_auth!(
            get_token_from_md(request.metadata()),
            "Token authentication error"
        );

        let user_id = DieselUlid::from_str(&request.get_ref().user_id)
            .map_err(|_| tonic::Status::invalid_argument("Invalid ulid"))?;
        let resource_id = DieselUlid::from_str(&request.get_ref().resource_id)
            .map_err(|_| tonic::Status::invalid_argument("Invalid ulid"))?;

        let ctx = Context::res_ctx(
            resource_id,
            crate::database::enums::DbPermissionLevel::ADMIN,
            false,
        );

        tonic_auth!(
            self.authorizer.check_permissions(&token, vec![ctx]).await,
            "Unauthorized"
        );

        let obj = self
            .cache
            .get_object(&resource_id)
            .ok_or_else(|| tonic::Status::not_found("Resource not found"))?;

        let user = tonic_internal!(
            self.database_handler
                .add_permission_to_user(
                    user_id,
                    resource_id,
                    obj.into_object_mapping(tonic_invalid!(
                        DbPermissionLevel::try_from(request.get_ref().permission_level),
                        "Invalid permission level"
                    ))
                )
                .await,
            "Internal error"
        );

        let resp = CreateAuthorizationResponse {
            resource_id: resource_id.to_string(),
            user_id: user_id.to_string(),
            user_name: user.display_name,
            permission_level: request.into_inner().permission_level,
        };

        return_with_log!(resp);
    }
    /// GetAuthorization
    ///
    /// Status: BETA
    ///
    /// This gets resource specific user authorizations
    async fn get_authorizations(
        &self,
        _request: tonic::Request<GetAuthorizationsRequest>,
    ) -> std::result::Result<tonic::Response<GetAuthorizationsResponse>, tonic::Status> {
        todo!()
    }
    /// DeleteAuthorization
    ///
    /// Status: BETA
    ///
    /// This creates a user-specific attribute that handles permission for a
    /// specific resource
    async fn delete_authorization(
        &self,
        _request: tonic::Request<DeleteAuthorizationRequest>,
    ) -> std::result::Result<tonic::Response<DeleteAuthorizationResponse>, tonic::Status> {
        todo!()
    }
    /// UpdateAuthorization
    ///
    /// Status: BETA
    ///
    /// This creates a user-specific attribute that handles permission for a
    /// specific resource
    async fn update_authorizations(
        &self,
        _request: tonic::Request<UpdateAuthorizationsRequest>,
    ) -> std::result::Result<tonic::Response<UpdateAuthorizationsResponse>, tonic::Status> {
        todo!()
    }
}