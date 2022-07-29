// Models
// These are the models for the above described requests and responses.
// gRPC best practises advice each Request and Response message in a RPC to be called {rpc_name}Request and {rpc_name}Response.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Url {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageObject {
    #[prost(string, tag="1")]
    pub filename: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub content_len: i64,
    #[prost(message, repeated, tag="5")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    #[prost(message, repeated, tag="6")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeNewObjectRequest {
    /// This describes the object to be initialized.
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<StageObject>,
    /// Collection id of the collection to which the object will be added.
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Should the object be uploaded via multipart?
    #[prost(bool, tag="3")]
    pub multipart: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeNewObjectResponse {
    /// ObjectId 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Staging ID, a generic ID when multipart is not enabled, otherwise the multipart upload ID.
    #[prost(string, tag="2")]
    pub staging_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUploadUrlRequest {
    /// ObjectId 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Staging ID, a generic ID when multipart is not enabled, otherwise the multipart upload ID.
    #[prost(string, tag="2")]
    pub staging_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    /// Is this a multipart upload?
    /// (optional) if multi was initialized 
    #[prost(int32, tag="4")]
    pub part_number: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUploadUrlResponse {
    /// URL
    #[prost(message, optional, tag="1")]
    pub url: ::core::option::Option<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedParts {
    /// Multipart identifier
    #[prost(string, tag="1")]
    pub etag: ::prost::alloc::string::String,
    /// Part number
    #[prost(int64, tag="2")]
    pub part: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectRevision {
    /// ObjectId
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// (optional) revision number, will be latest if not set.
    #[prost(int64, tag="2")]
    pub revision: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadUrlRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<ObjectRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadUrlResponse {
    #[prost(message, optional, tag="1")]
    pub url: ::core::option::Option<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadLinksBatchRequest {
    /// CollectionID
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// ObjectIds
    #[prost(message, repeated, tag="2")]
    pub objects: ::prost::alloc::vec::Vec<ObjectRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadLinksBatchResponse {
    #[prost(message, repeated, tag="1")]
    pub urls: ::prost::alloc::vec::Vec<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinksStreamRequest {
    /// CollectionID
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// ObjectIds
    #[prost(message, repeated, tag="2")]
    pub objects: ::prost::alloc::vec::Vec<ObjectRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinksStreamResponse {
    #[prost(message, optional, tag="1")]
    pub url: ::core::option::Option<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectStagingRequest {
    /// ObjectId 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Staging ID, a generic ID when multipart is not enabled, otherwise the multipart upload ID.
    #[prost(string, tag="2")]
    pub staging_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    /// Hash of the uploaded data - used to verify the data integrity.
    /// This supports multiple hashing algorithms.
    #[prost(message, optional, tag="4")]
    pub hash: ::core::option::Option<super::super::models::v1::Hash>,
    /// If the upload was multipart, this is the list of parts that were uploaded.
    /// Should be empty if the upload was not multipart.
    /// (optional)
    #[prost(message, repeated, tag="5")]
    pub completed_parts: ::prost::alloc::vec::Vec<CompletedParts>,
    /// Should the object be auto-updated in the owner collection?
    /// default: false
    #[prost(bool, tag="6")]
    pub auto_update: bool,
    /// Add the object automatically to these collections
    /// (optional)
    #[prost(string, repeated, tag="7")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectStagingResponse {
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectRequest {
    /// Existing object ID
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// collection ID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// New object data
    #[prost(message, optional, tag="3")]
    pub object: ::core::option::Option<StageObject>,
    /// Should new data be uploaded ?
    #[prost(bool, tag="4")]
    pub reupload: bool,
    /// Should a multipart upload be used?
    #[prost(bool, tag="5")]
    pub multi_part: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectResponse {
    /// ObjectId 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Staging ID, a generic ID when multipart is not enabled, otherwise the multipart upload ID.
    #[prost(string, tag="2")]
    pub staging_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowObjectRequest {
    /// ObjectId 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// OwnerCollectionID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// BorrowerCollectionID
    #[prost(string, tag="3")]
    pub target_collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowObjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectRequest {
    /// ObjectId 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectResponse {
    /// This describes the new object.
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    /// ObjectId 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// This will by default delete the object only in the specified collection.
    /// If the collection_id is the owner of the object, cascading=true will delete the object in all collections.
    /// Default: false
    #[prost(bool, tag="3")]
    pub cascade: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectWithUrl {
    /// Description of a specified object
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
    /// This is a associated download URL
    /// Will be empty if request does not contain the associated with_url flag
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectByIdRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub revision: i64,
    /// With URL: Include URL in response ?
    #[prost(bool, tag="4")]
    pub with_url: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectByIdResponse {
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<ObjectWithUrl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Paginate the results: Default is 20
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    /// Filter by Labels (optional) OR request a specific list of Objects
    #[prost(message, optional, tag="3")]
    pub label_id_filter: ::core::option::Option<super::super::models::v1::LabelOrIdQuery>,
    /// With URL: Include URL in response ?
    #[prost(bool, tag="4")]
    pub with_url: bool,
    /// Should this request consider older revisions of Objects ?
    #[prost(bool, tag="5")]
    pub include_history: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsResponse {
    /// A List of objects with (optional) associated URLs
    #[prost(message, repeated, tag="1")]
    pub objects: ::prost::alloc::vec::Vec<ObjectWithUrl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectHistoryByIdRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    #[prost(bool, tag="4")]
    pub with_url: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectHistoryByIdResponse {
    #[prost(message, repeated, tag="1")]
    pub objects: ::prost::alloc::vec::Vec<ObjectWithUrl>,
}
/// Generated client implementations.
pub mod object_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ObjectServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ObjectServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ObjectServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ObjectServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// This initializes a new object
        /// Initializing an object will put it in a staging area.
        /// Staged objects will get a separate staging id and need to be finished before they can be used.
        pub async fn initialize_new_object(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializeNewObjectRequest>,
        ) -> Result<tonic::Response<super::InitializeNewObjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/InitializeNewObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method will return a (multi-part) url that can be used to upload a file to S3.
        /// Part is a optional query parameter that can be used to upload a part of the file / multipart upload.
        pub async fn get_upload_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUploadUrlRequest>,
        ) -> Result<tonic::Response<super::GetUploadUrlResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/GetUploadURL",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method will return a url that can be used to download a file from S3.
        pub async fn get_download_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDownloadUrlRequest>,
        ) -> Result<tonic::Response<super::GetDownloadUrlResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadURL",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method can be used to get download urls for multiple objects.
        /// The order of the returned urls will be the same as the order of the object ids in the request.
        pub async fn get_download_links_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDownloadLinksBatchRequest>,
        ) -> Result<
            tonic::Response<super::GetDownloadLinksBatchResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadLinksBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a stream of objects and presigned links based on the provided query
        /// This can be used retrieve a large number of Objects as a stream that would otherwise cause issues with the connection
        pub async fn create_download_links_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinksStreamRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::CreateDownloadLinksStreamResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/CreateDownloadLinksStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// This method completes the staging of an object.
        pub async fn finish_object_staging(
            &mut self,
            request: impl tonic::IntoRequest<super::FinishObjectStagingRequest>,
        ) -> Result<tonic::Response<super::FinishObjectStagingResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/FinishObjectStaging",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Objects are immutable!
        /// Updating an object will create a new revision for the object
        /// This method will put the new revision in a staging area.
        /// Staged objects will get a separate staging id and need to be finished before they can be used.
        pub async fn update_object(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/UpdateObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method will borrow an object to another collection.
        /// This can only be used if the object is owned by the current collection.
        /// Borrowed objects are references and cannot be updated directly.
        /// Updating a borrowed object will clone the object and create a copy in the new collection.
        /// This copy will not receive any updates from the original object.
        /// The original owner will be referenced in the origin section of the object.
        /// This owner can delete the object even if it was cloned
        pub async fn borrow_object(
            &mut self,
            request: impl tonic::IntoRequest<super::BorrowObjectRequest>,
        ) -> Result<tonic::Response<super::BorrowObjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/BorrowObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method clones an object and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from the original object.
        pub async fn clone_object(
            &mut self,
            request: impl tonic::IntoRequest<super::CloneObjectRequest>,
        ) -> Result<tonic::Response<super::CloneObjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/CloneObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the object with the complete revision history.
        /// This should be avoided if possible.
        /// This method allows the owner to cascade the deletion of all objects that were cloned from this object.
        /// -> GDPR compliant procedure.
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/DeleteObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectByID gets a specific Object by ID that is associated to the current collection
        /// By default only the latest revision of an object will be returned
        /// Specify a revision_number to select an older revision
        /// With the optional with_url boolean a download link can automatically be requested
        pub async fn get_object_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectByIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/GetObjectByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjects returns a (paginated) list of objects in a specific collection
        /// By default only the latest revisions of all objects will be shown
        /// This behaviour can be changed with the include_history flag
        /// With the optional with_url boolean a download link can automatically be requested for each Object
        /// This request contains a LabelOrIDQuery message, this is either a list of request ObjectIDs
        /// or a query filtered by Labels
        pub async fn get_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectsRequest>,
        ) -> Result<tonic::Response<super::GetObjectsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/GetObjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectHistoryByID returns the full history of a specified object
        /// With the optional with_url boolean a download link can automatically be requested for each Object
        /// This is by default a paginated request
        pub async fn get_object_history_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectHistoryByIdRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectHistoryByIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectService/GetObjectHistoryByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod object_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ObjectServiceServer.
    #[async_trait]
    pub trait ObjectService: Send + Sync + 'static {
        /// This initializes a new object
        /// Initializing an object will put it in a staging area.
        /// Staged objects will get a separate staging id and need to be finished before they can be used.
        async fn initialize_new_object(
            &self,
            request: tonic::Request<super::InitializeNewObjectRequest>,
        ) -> Result<tonic::Response<super::InitializeNewObjectResponse>, tonic::Status>;
        /// This method will return a (multi-part) url that can be used to upload a file to S3.
        /// Part is a optional query parameter that can be used to upload a part of the file / multipart upload.
        async fn get_upload_url(
            &self,
            request: tonic::Request<super::GetUploadUrlRequest>,
        ) -> Result<tonic::Response<super::GetUploadUrlResponse>, tonic::Status>;
        /// This method will return a url that can be used to download a file from S3.
        async fn get_download_url(
            &self,
            request: tonic::Request<super::GetDownloadUrlRequest>,
        ) -> Result<tonic::Response<super::GetDownloadUrlResponse>, tonic::Status>;
        /// This method can be used to get download urls for multiple objects.
        /// The order of the returned urls will be the same as the order of the object ids in the request.
        async fn get_download_links_batch(
            &self,
            request: tonic::Request<super::GetDownloadLinksBatchRequest>,
        ) -> Result<
            tonic::Response<super::GetDownloadLinksBatchResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the CreateDownloadLinksStream method.
        type CreateDownloadLinksStreamStream: futures_core::Stream<
                Item = Result<super::CreateDownloadLinksStreamResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Creates a stream of objects and presigned links based on the provided query
        /// This can be used retrieve a large number of Objects as a stream that would otherwise cause issues with the connection
        async fn create_download_links_stream(
            &self,
            request: tonic::Request<super::CreateDownloadLinksStreamRequest>,
        ) -> Result<
            tonic::Response<Self::CreateDownloadLinksStreamStream>,
            tonic::Status,
        >;
        /// This method completes the staging of an object.
        async fn finish_object_staging(
            &self,
            request: tonic::Request<super::FinishObjectStagingRequest>,
        ) -> Result<tonic::Response<super::FinishObjectStagingResponse>, tonic::Status>;
        /// Objects are immutable!
        /// Updating an object will create a new revision for the object
        /// This method will put the new revision in a staging area.
        /// Staged objects will get a separate staging id and need to be finished before they can be used.
        async fn update_object(
            &self,
            request: tonic::Request<super::UpdateObjectRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectResponse>, tonic::Status>;
        /// This method will borrow an object to another collection.
        /// This can only be used if the object is owned by the current collection.
        /// Borrowed objects are references and cannot be updated directly.
        /// Updating a borrowed object will clone the object and create a copy in the new collection.
        /// This copy will not receive any updates from the original object.
        /// The original owner will be referenced in the origin section of the object.
        /// This owner can delete the object even if it was cloned
        async fn borrow_object(
            &self,
            request: tonic::Request<super::BorrowObjectRequest>,
        ) -> Result<tonic::Response<super::BorrowObjectResponse>, tonic::Status>;
        /// This method clones an object and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from the original object.
        async fn clone_object(
            &self,
            request: tonic::Request<super::CloneObjectRequest>,
        ) -> Result<tonic::Response<super::CloneObjectResponse>, tonic::Status>;
        /// Deletes the object with the complete revision history.
        /// This should be avoided if possible.
        /// This method allows the owner to cascade the deletion of all objects that were cloned from this object.
        /// -> GDPR compliant procedure.
        async fn delete_object(
            &self,
            request: tonic::Request<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status>;
        /// GetObjectByID gets a specific Object by ID that is associated to the current collection
        /// By default only the latest revision of an object will be returned
        /// Specify a revision_number to select an older revision
        /// With the optional with_url boolean a download link can automatically be requested
        async fn get_object_by_id(
            &self,
            request: tonic::Request<super::GetObjectByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectByIdResponse>, tonic::Status>;
        /// GetObjects returns a (paginated) list of objects in a specific collection
        /// By default only the latest revisions of all objects will be shown
        /// This behaviour can be changed with the include_history flag
        /// With the optional with_url boolean a download link can automatically be requested for each Object
        /// This request contains a LabelOrIDQuery message, this is either a list of request ObjectIDs
        /// or a query filtered by Labels
        async fn get_objects(
            &self,
            request: tonic::Request<super::GetObjectsRequest>,
        ) -> Result<tonic::Response<super::GetObjectsResponse>, tonic::Status>;
        /// GetObjectHistoryByID returns the full history of a specified object
        /// With the optional with_url boolean a download link can automatically be requested for each Object
        /// This is by default a paginated request
        async fn get_object_history_by_id(
            &self,
            request: tonic::Request<super::GetObjectHistoryByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectHistoryByIdResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ObjectServiceServer<T: ObjectService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectService> ObjectServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectServiceServer<T>
    where
        T: ObjectService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.ObjectService/InitializeNewObject" => {
                    #[allow(non_camel_case_types)]
                    struct InitializeNewObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::InitializeNewObjectRequest>
                    for InitializeNewObjectSvc<T> {
                        type Response = super::InitializeNewObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializeNewObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).initialize_new_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitializeNewObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/GetUploadURL" => {
                    #[allow(non_camel_case_types)]
                    struct GetUploadURLSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetUploadUrlRequest>
                    for GetUploadURLSvc<T> {
                        type Response = super::GetUploadUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUploadUrlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_upload_url(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUploadURLSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadURL" => {
                    #[allow(non_camel_case_types)]
                    struct GetDownloadURLSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetDownloadUrlRequest>
                    for GetDownloadURLSvc<T> {
                        type Response = super::GetDownloadUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDownloadUrlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_download_url(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDownloadURLSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadLinksBatch" => {
                    #[allow(non_camel_case_types)]
                    struct GetDownloadLinksBatchSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetDownloadLinksBatchRequest>
                    for GetDownloadLinksBatchSvc<T> {
                        type Response = super::GetDownloadLinksBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDownloadLinksBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_download_links_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDownloadLinksBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/CreateDownloadLinksStream" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinksStreamSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::ServerStreamingService<
                        super::CreateDownloadLinksStreamRequest,
                    > for CreateDownloadLinksStreamSvc<T> {
                        type Response = super::CreateDownloadLinksStreamResponse;
                        type ResponseStream = T::CreateDownloadLinksStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateDownloadLinksStreamRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_download_links_stream(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDownloadLinksStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/FinishObjectStaging" => {
                    #[allow(non_camel_case_types)]
                    struct FinishObjectStagingSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::FinishObjectStagingRequest>
                    for FinishObjectStagingSvc<T> {
                        type Response = super::FinishObjectStagingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinishObjectStagingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).finish_object_staging(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinishObjectStagingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/UpdateObject" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::UpdateObjectRequest>
                    for UpdateObjectSvc<T> {
                        type Response = super::UpdateObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/BorrowObject" => {
                    #[allow(non_camel_case_types)]
                    struct BorrowObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::BorrowObjectRequest>
                    for BorrowObjectSvc<T> {
                        type Response = super::BorrowObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BorrowObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).borrow_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BorrowObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/CloneObject" => {
                    #[allow(non_camel_case_types)]
                    struct CloneObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::CloneObjectRequest>
                    for CloneObjectSvc<T> {
                        type Response = super::CloneObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloneObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).clone_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloneObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/DeleteObject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::DeleteObjectRequest>
                    for DeleteObjectSvc<T> {
                        type Response = super::DeleteObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/GetObjectByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectByIDSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectByIdRequest>
                    for GetObjectByIDSvc<T> {
                        type Response = super::GetObjectByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/GetObjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectsSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectsRequest>
                    for GetObjectsSvc<T> {
                        type Response = super::GetObjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_objects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/GetObjectHistoryByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectHistoryByIDSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectHistoryByIdRequest>
                    for GetObjectHistoryByIDSvc<T> {
                        type Response = super::GetObjectHistoryByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectHistoryByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_history_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectHistoryByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ObjectService> Clone for ObjectServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObjectService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectService> tonic::transport::NamedService for ObjectServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.ObjectService";
    }
}
// Models
// This section contains the models for each individual Request and corresponding Response

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    /// This is the reference to the Objects that should be added to the group
    #[prost(message, repeated, tag="4")]
    pub objects: ::prost::alloc::vec::Vec<super::super::models::v1::Object>,
    /// This is a reference to the Objects that are associated with "meta" data about corresponding objects in the group
    #[prost(message, repeated, tag="5")]
    pub meta_objects: ::prost::alloc::vec::Vec<super::super::models::v1::Object>,
    #[prost(message, repeated, tag="6")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    #[prost(message, repeated, tag="7")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupResponse {
    #[prost(string, tag="1")]
    pub object_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectGroupRequest {
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub collection_id: ::prost::alloc::string::String,
    /// This is the reference to the Objects that should be added to the group
    #[prost(message, repeated, tag="5")]
    pub objects: ::prost::alloc::vec::Vec<super::super::models::v1::Object>,
    /// This is a reference to the Objects that are associated with "meta" data about corresponding objects in the group
    #[prost(message, repeated, tag="6")]
    pub meta_objects: ::prost::alloc::vec::Vec<super::super::models::v1::Object>,
    #[prost(message, repeated, tag="7")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    #[prost(message, repeated, tag="8")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectGroupResponse {
    #[prost(message, optional, tag="1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupByIdRequest {
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Optional revision
    #[prost(int64, tag="3")]
    pub revision: i64,
    #[prost(enumeration="super::super::models::v1::OutputFormat", tag="4")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupByIdResponse {
    #[prost(oneof="get_object_group_by_id_response::ObjectGroup", tags="1, 2, 3")]
    pub object_group: ::core::option::Option<get_object_group_by_id_response::ObjectGroup>,
}
/// Nested message and enum types in `GetObjectGroupByIdResponse`.
pub mod get_object_group_by_id_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectGroup {
        #[prost(message, tag="1")]
        ObjectGroupOverview(super::super::super::models::v1::ObjectGroupOverview),
        #[prost(message, tag="2")]
        ObjectGroupWithId(super::super::super::models::v1::ObjectGroupWithId),
        #[prost(message, tag="3")]
        ObjectGroupFull(super::super::super::models::v1::ObjectGroup),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsFromObjectRequest {
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    #[prost(enumeration="super::super::models::v1::OutputFormat", tag="4")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsFromObjectResponse {
    #[prost(oneof="get_object_groups_from_object_response::ObjectGroups", tags="1, 2, 3")]
    pub object_groups: ::core::option::Option<get_object_groups_from_object_response::ObjectGroups>,
}
/// Nested message and enum types in `GetObjectGroupsFromObjectResponse`.
pub mod get_object_groups_from_object_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectGroups {
        #[prost(message, tag="1")]
        ObjectGroupsOverview(super::super::super::models::v1::ObjectGroupOverviews),
        #[prost(message, tag="2")]
        ObjectGroupsWithId(super::super::super::models::v1::ObjectGroupWithIDs),
        #[prost(message, tag="3")]
        ObjectGroupsFull(super::super::super::models::v1::ObjectGroups),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupRequest {
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Optional revision
    #[prost(int64, tag="3")]
    pub revision: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Paginate the results: Default is 20
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    /// Filter by Labels (optional) OR request a specific list of ObjectGroups
    #[prost(message, optional, tag="3")]
    pub label_id_filter: ::core::option::Option<super::super::models::v1::LabelOrIdQuery>,
    #[prost(enumeration="super::super::models::v1::OutputFormat", tag="4")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsResponse {
    #[prost(oneof="get_object_groups_response::ObjectGroups", tags="1, 2, 3")]
    pub object_groups: ::core::option::Option<get_object_groups_response::ObjectGroups>,
}
/// Nested message and enum types in `GetObjectGroupsResponse`.
pub mod get_object_groups_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectGroups {
        #[prost(message, tag="1")]
        ObjectGroupsOverview(super::super::super::models::v1::ObjectGroupOverviews),
        #[prost(message, tag="2")]
        ObjectGroupsWithId(super::super::super::models::v1::ObjectGroupWithIDs),
        #[prost(message, tag="3")]
        ObjectGroupsFull(super::super::super::models::v1::ObjectGroups),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupHistoryByIdRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    #[prost(enumeration="super::super::models::v1::OutputFormat", tag="4")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupHistoryByIdResponse {
    #[prost(oneof="get_object_group_history_by_id_response::ObjectGroups", tags="1, 2, 3")]
    pub object_groups: ::core::option::Option<get_object_group_history_by_id_response::ObjectGroups>,
}
/// Nested message and enum types in `GetObjectGroupHistoryByIDResponse`.
pub mod get_object_group_history_by_id_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectGroups {
        #[prost(message, tag="1")]
        ObjectGroupsOverview(super::super::super::models::v1::ObjectGroupOverviews),
        #[prost(message, tag="2")]
        ObjectGroupsWithId(super::super::super::models::v1::ObjectGroupWithIDs),
        #[prost(message, tag="3")]
        ObjectGroupsFull(super::super::super::models::v1::ObjectGroups),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowObjectGroupRequest {
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub target_collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowObjectGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectGroupRequest {
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectGroupResponse {
    #[prost(message, optional, tag="1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroupOverview>,
}
/// Generated client implementations.
pub mod object_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ObjectGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectGroupServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ObjectGroupServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ObjectGroupServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ObjectGroupServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// CreateObjectGroup creates a new ObjectGroup in the collection
        pub async fn create_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/CreateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateObjectGroup creates an updated ObjectGroup
        /// ObjectGroups are immutable
        /// Updating an ObjectGroup will create a new Revision of the ObjectGroup
        pub async fn update_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectGroupResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/UpdateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroupById gets a specific ObjectGroup by ID
        /// By default the latest revision is always returned, older revisions need to be specified separately
        pub async fn get_object_group_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupByIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroupsFromObject gets all ObjectGroups associated to a specific Object
        /// Objects can be part of multiple ObjectGroups at once
        pub async fn get_object_groups_from_object(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupsFromObjectRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupsFromObjectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupsFromObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroups is a request that returns a (paginated) list of ObjectGroups that contain a specific set of labels.
        pub async fn get_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_group_history_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupHistoryByIdRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupHistoryByIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupHistoryByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteObjectGroup is a request that deletes a specified ObjectGroup
        /// This does not delete the associated Objects
        pub async fn delete_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/DeleteObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method will borrow an ObjectGroup to another collection.
        /// Borrowed ObjectGroup are references and cannot be updated directly.
        /// Borrowing an ObjectGroup will also automatically borrow all associated Objects to the new collection.
        /// Updating a borrowed ObjectGroup will clone the ObjectGroup and create a copy in the new collection.
        /// This is equivalent to the CloneObjectGroup rpc.
        /// This copy will not receive any updates from the original ObjectGroup.
        pub async fn borrow_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::BorrowObjectGroupRequest>,
        ) -> Result<tonic::Response<super::BorrowObjectGroupResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/BorrowObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method clones an ObjectGroup and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from the original ObjectGroup.
        pub async fn clone_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CloneObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CloneObjectGroupResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.ObjectGroupService/CloneObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod object_group_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ObjectGroupServiceServer.
    #[async_trait]
    pub trait ObjectGroupService: Send + Sync + 'static {
        /// CreateObjectGroup creates a new ObjectGroup in the collection
        async fn create_object_group(
            &self,
            request: tonic::Request<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status>;
        /// UpdateObjectGroup creates an updated ObjectGroup
        /// ObjectGroups are immutable
        /// Updating an ObjectGroup will create a new Revision of the ObjectGroup
        async fn update_object_group(
            &self,
            request: tonic::Request<super::UpdateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectGroupResponse>, tonic::Status>;
        /// GetObjectGroupById gets a specific ObjectGroup by ID
        /// By default the latest revision is always returned, older revisions need to be specified separately
        async fn get_object_group_by_id(
            &self,
            request: tonic::Request<super::GetObjectGroupByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupByIdResponse>, tonic::Status>;
        /// GetObjectGroupsFromObject gets all ObjectGroups associated to a specific Object
        /// Objects can be part of multiple ObjectGroups at once
        async fn get_object_groups_from_object(
            &self,
            request: tonic::Request<super::GetObjectGroupsFromObjectRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupsFromObjectResponse>,
            tonic::Status,
        >;
        /// GetObjectGroups is a request that returns a (paginated) list of ObjectGroups that contain a specific set of labels.
        async fn get_object_groups(
            &self,
            request: tonic::Request<super::GetObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsResponse>, tonic::Status>;
        async fn get_object_group_history_by_id(
            &self,
            request: tonic::Request<super::GetObjectGroupHistoryByIdRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupHistoryByIdResponse>,
            tonic::Status,
        >;
        /// DeleteObjectGroup is a request that deletes a specified ObjectGroup
        /// This does not delete the associated Objects
        async fn delete_object_group(
            &self,
            request: tonic::Request<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status>;
        /// This method will borrow an ObjectGroup to another collection.
        /// Borrowed ObjectGroup are references and cannot be updated directly.
        /// Borrowing an ObjectGroup will also automatically borrow all associated Objects to the new collection.
        /// Updating a borrowed ObjectGroup will clone the ObjectGroup and create a copy in the new collection.
        /// This is equivalent to the CloneObjectGroup rpc.
        /// This copy will not receive any updates from the original ObjectGroup.
        async fn borrow_object_group(
            &self,
            request: tonic::Request<super::BorrowObjectGroupRequest>,
        ) -> Result<tonic::Response<super::BorrowObjectGroupResponse>, tonic::Status>;
        /// This method clones an ObjectGroup and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from the original ObjectGroup.
        async fn clone_object_group(
            &self,
            request: tonic::Request<super::CloneObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CloneObjectGroupResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ObjectGroupServiceServer<T: ObjectGroupService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectGroupService> ObjectGroupServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectGroupServiceServer<T>
    where
        T: ObjectGroupService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.ObjectGroupService/CreateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::CreateObjectGroupRequest>
                    for CreateObjectGroupSvc<T> {
                        type Response = super::CreateObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/UpdateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::UpdateObjectGroupRequest>
                    for UpdateObjectGroupSvc<T> {
                        type Response = super::UpdateObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupById" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupByIdSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::GetObjectGroupByIdRequest>
                    for GetObjectGroupByIdSvc<T> {
                        type Response = super::GetObjectGroupByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_group_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupsFromObject" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupsFromObjectSvc<T: ObjectGroupService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<
                        super::GetObjectGroupsFromObjectRequest,
                    > for GetObjectGroupsFromObjectSvc<T> {
                        type Response = super::GetObjectGroupsFromObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetObjectGroupsFromObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_groups_from_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupsFromObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupsSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::GetObjectGroupsRequest>
                    for GetObjectGroupsSvc<T> {
                        type Response = super::GetObjectGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_groups(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupHistoryByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupHistoryByIDSvc<T: ObjectGroupService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<
                        super::GetObjectGroupHistoryByIdRequest,
                    > for GetObjectGroupHistoryByIDSvc<T> {
                        type Response = super::GetObjectGroupHistoryByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetObjectGroupHistoryByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_group_history_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupHistoryByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/DeleteObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::DeleteObjectGroupRequest>
                    for DeleteObjectGroupSvc<T> {
                        type Response = super::DeleteObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/BorrowObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct BorrowObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::BorrowObjectGroupRequest>
                    for BorrowObjectGroupSvc<T> {
                        type Response = super::BorrowObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BorrowObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).borrow_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BorrowObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectGroupService/CloneObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CloneObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::CloneObjectGroupRequest>
                    for CloneObjectGroupSvc<T> {
                        type Response = super::CloneObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloneObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).clone_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloneObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ObjectGroupService> Clone for ObjectGroupServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObjectGroupService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectGroupService> tonic::transport::NamedService
    for ObjectGroupServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.ObjectGroupService";
    }
}
// Models:

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNewCollectionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    #[prost(message, repeated, tag="4")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    #[prost(message, repeated, tag="5")]
    pub authorization: ::prost::alloc::vec::Vec<super::super::models::v1::Authorization>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNewCollectionResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::models::v1::OutputFormat", tag="2")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionByIdResponse {
    #[prost(oneof="get_collection_by_id_response::Collection", tags="1, 2, 3")]
    pub collection: ::core::option::Option<get_collection_by_id_response::Collection>,
}
/// Nested message and enum types in `GetCollectionByIDResponse`.
pub mod get_collection_by_id_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Collection {
        #[prost(message, tag="1")]
        CollectionOverview(super::super::super::models::v1::CollectionOverview),
        #[prost(message, tag="2")]
        CollectionWithId(super::super::super::models::v1::CollectionWithId),
        #[prost(message, tag="3")]
        CollectionFull(super::super::super::models::v1::Collection),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionsRequest {
    /// Filter by Labels (optional) OR request a specific list of Collections
    #[prost(message, optional, tag="1")]
    pub label_filter: ::core::option::Option<super::super::models::v1::LabelOrIdQuery>,
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    #[prost(enumeration="super::super::models::v1::OutputFormat", tag="3")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionsResponse {
    /// These are plural representations of their specific single counterparts
    #[prost(oneof="get_collections_response::Collections", tags="1, 2, 3")]
    pub collections: ::core::option::Option<get_collections_response::Collections>,
}
/// Nested message and enum types in `GetCollectionsResponse`.
pub mod get_collections_response {
    /// These are plural representations of their specific single counterparts
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Collections {
        #[prost(message, tag="1")]
        CollectionsOverview(super::super::super::models::v1::CollectionOverviews),
        #[prost(message, tag="2")]
        CollectionsWithId(super::super::super::models::v1::CollectionWithIDs),
        #[prost(message, tag="3")]
        CollectionsFull(super::super::super::models::v1::Collections),
    }
}
/// This updates the collection
/// Updating a pinned collection will require a new version to be created
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    #[prost(message, repeated, tag="5")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    #[prost(message, repeated, tag="6")]
    pub authorization: ::prost::alloc::vec::Vec<super::super::models::v1::Authorization>,
    /// If this is set, the collection will be automatically pinned to this version
    /// Similar to the more explicit Pin request
    /// Updating a pinned collection will make this field required
    /// (optional if unpinned || required if pinned)
    #[prost(message, optional, tag="7")]
    pub version: ::core::option::Option<super::super::models::v1::Version>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionResponse {
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<super::super::models::v1::CollectionOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinCollectionVersionRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub version: ::core::option::Option<super::super::models::v1::Version>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinCollectionVersionResponse {
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<super::super::models::v1::CollectionOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Should the collection and all associated versions be deleted ?
    #[prost(bool, tag="2")]
    pub with_versions: bool,
    /// Should the deletion cascade to all owned objects ?
    #[prost(bool, tag="3")]
    pub cascade: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionResponse {
}
/// Generated client implementations.
pub mod collection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CollectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CollectionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CollectionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CollectionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CollectionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// CreateNewCollection creates a new Collection
        pub async fn create_new_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNewCollectionRequest>,
        ) -> Result<tonic::Response<super::CreateNewCollectionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.CollectionService/CreateNewCollection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetCollection queries a specific Collection by ID
        /// The result can be one_of:
        /// CollectionOverview -> default
        /// CollectionWithID
        /// Collection (full)
        /// This can be modified with the optional OutputFormat parameter
        pub async fn get_collection_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionByIdRequest>,
        ) -> Result<tonic::Response<super::GetCollectionByIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.CollectionService/GetCollectionByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetCollections queries multiple collections by ID or by LabelFilter
        /// This returns by default a paginated result with 20 entries.
        pub async fn get_collections(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionsRequest>,
        ) -> Result<tonic::Response<super::GetCollectionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.CollectionService/GetCollections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateCollection updates the current collection
        /// This will update the collection in place if it is unversioned / latest
        /// A versioned (pinned) collection requires a new semantic version after the update
        /// This can be used to pin a collection to a specific version
        /// similar to the PinCollectionVersion request
        pub async fn update_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectionRequest>,
        ) -> Result<tonic::Response<super::UpdateCollectionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.CollectionService/UpdateCollection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PinCollectionVersion this pins the current status of the version to a specific version
        /// This effectively creates a copy of the collection with a stable version
        /// All objects will be pinned to an explicit revision number
        /// Pinned collections can not be updated in place
        pub async fn pin_collection_version(
            &mut self,
            request: impl tonic::IntoRequest<super::PinCollectionVersionRequest>,
        ) -> Result<
            tonic::Response<super::PinCollectionVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.CollectionService/PinCollectionVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This request deletes the collection.
        /// If with_version is true, it deletes the collection and all its versions.
        /// If cascade is true, all objects that are owned by the collection will also deleted.
        /// This should be avoided
        pub async fn delete_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.CollectionService/DeleteCollection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod collection_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CollectionServiceServer.
    #[async_trait]
    pub trait CollectionService: Send + Sync + 'static {
        /// CreateNewCollection creates a new Collection
        async fn create_new_collection(
            &self,
            request: tonic::Request<super::CreateNewCollectionRequest>,
        ) -> Result<tonic::Response<super::CreateNewCollectionResponse>, tonic::Status>;
        /// GetCollection queries a specific Collection by ID
        /// The result can be one_of:
        /// CollectionOverview -> default
        /// CollectionWithID
        /// Collection (full)
        /// This can be modified with the optional OutputFormat parameter
        async fn get_collection_by_id(
            &self,
            request: tonic::Request<super::GetCollectionByIdRequest>,
        ) -> Result<tonic::Response<super::GetCollectionByIdResponse>, tonic::Status>;
        /// GetCollections queries multiple collections by ID or by LabelFilter
        /// This returns by default a paginated result with 20 entries.
        async fn get_collections(
            &self,
            request: tonic::Request<super::GetCollectionsRequest>,
        ) -> Result<tonic::Response<super::GetCollectionsResponse>, tonic::Status>;
        /// UpdateCollection updates the current collection
        /// This will update the collection in place if it is unversioned / latest
        /// A versioned (pinned) collection requires a new semantic version after the update
        /// This can be used to pin a collection to a specific version
        /// similar to the PinCollectionVersion request
        async fn update_collection(
            &self,
            request: tonic::Request<super::UpdateCollectionRequest>,
        ) -> Result<tonic::Response<super::UpdateCollectionResponse>, tonic::Status>;
        /// PinCollectionVersion this pins the current status of the version to a specific version
        /// This effectively creates a copy of the collection with a stable version
        /// All objects will be pinned to an explicit revision number
        /// Pinned collections can not be updated in place
        async fn pin_collection_version(
            &self,
            request: tonic::Request<super::PinCollectionVersionRequest>,
        ) -> Result<tonic::Response<super::PinCollectionVersionResponse>, tonic::Status>;
        /// This request deletes the collection.
        /// If with_version is true, it deletes the collection and all its versions.
        /// If cascade is true, all objects that are owned by the collection will also deleted.
        /// This should be avoided
        async fn delete_collection(
            &self,
            request: tonic::Request<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CollectionServiceServer<T: CollectionService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CollectionService> CollectionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CollectionServiceServer<T>
    where
        T: CollectionService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.CollectionService/CreateNewCollection" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNewCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::CreateNewCollectionRequest>
                    for CreateNewCollectionSvc<T> {
                        type Response = super::CreateNewCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateNewCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_new_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateNewCollectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.CollectionService/GetCollectionByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetCollectionByIDSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::GetCollectionByIdRequest>
                    for GetCollectionByIDSvc<T> {
                        type Response = super::GetCollectionByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_collection_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCollectionByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.CollectionService/GetCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetCollectionsSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::GetCollectionsRequest>
                    for GetCollectionsSvc<T> {
                        type Response = super::GetCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_collections(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCollectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.CollectionService/UpdateCollection" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::UpdateCollectionRequest>
                    for UpdateCollectionSvc<T> {
                        type Response = super::UpdateCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateCollectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.CollectionService/PinCollectionVersion" => {
                    #[allow(non_camel_case_types)]
                    struct PinCollectionVersionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::PinCollectionVersionRequest>
                    for PinCollectionVersionSvc<T> {
                        type Response = super::PinCollectionVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PinCollectionVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).pin_collection_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PinCollectionVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.CollectionService/DeleteCollection" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::DeleteCollectionRequest>
                    for DeleteCollectionSvc<T> {
                        type Response = super::DeleteCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCollectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: CollectionService> Clone for CollectionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CollectionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CollectionService> tonic::transport::NamedService
    for CollectionServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.CollectionService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectRequest {
    /// The id of the project to add the user to
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub user_permission: ::core::option::Option<super::super::models::v1::ProjectPermission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenRequest {
    /// The id of the project to create the token for
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::models::v1::TokenType", tag="3")]
    pub token_type: i32,
    /// Empty if token_type is personal, otherwise the id of the collection to create the token for
    #[prost(string, repeated, tag="4")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::models::v1::Permission", tag="5")]
    pub permission: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenResponse {
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<super::super::models::v1::Token>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectCollectionsRequest {
    /// The id of the project to get the collections for
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectCollectionsResponse {
    #[prost(message, repeated, tag="1")]
    pub collection: ::prost::alloc::vec::Vec<super::super::models::v1::CollectionOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserCollectionsRequest {
    /// The id of the project to get the collections for
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserCollectionsResponse {
    #[prost(message, repeated, tag="1")]
    pub collection: ::prost::alloc::vec::Vec<super::super::models::v1::CollectionOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectRequest {
    /// The id of the project to get
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectResponse {
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<super::super::models::v1::ProjectOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokensRequest {
    /// The id of the project to get the API tokens for
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokensResponse {
    /// List of API tokens with redacted actual token
    #[prost(message, repeated, tag="1")]
    pub token: ::prost::alloc::vec::Vec<super::super::models::v1::Token>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenRequest {
    /// The id of the project to delete the API token for
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyProjectRequest {
    /// The id of the project to destroy
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyProjectResponse {
}
/// Generated client implementations.
pub mod auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuthServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AuthServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AuthServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// This creates a new authorization group.option
        /// All users and collections are bundled in a authorization group.
        pub async fn create_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/CreateProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///AddUserToProject Adds a new user to a given project by its id
        pub async fn add_user_to_project(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/AddUserToProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///CreateAPIToken Creates an API token to authenticate
        pub async fn create_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/CreateAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///GetProjectCollections Returns all collections that belong to a certain project
        pub async fn get_project_collections(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectCollectionsRequest>,
        ) -> Result<
            tonic::Response<super::GetProjectCollectionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/GetProjectCollections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///GetUserCollections Returns all collections that a specified user has access to
        pub async fn get_user_collections(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserCollectionsRequest>,
        ) -> Result<tonic::Response<super::GetUserCollectionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/GetUserCollections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///GetProject Returns the specified project
        pub async fn get_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/GetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all API token for a specific user
        pub async fn get_api_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiTokensRequest>,
        ) -> Result<tonic::Response<super::GetApiTokensResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/GetAPITokens",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///DeleteAPITokenRequest Deletes the specified API Token
        pub async fn delete_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/DeleteAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This will destroy the project and all its associated data.
        /// including users, collections, and API tokens and all data associated with them.
        pub async fn destroy_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyProjectRequest>,
        ) -> Result<tonic::Response<super::DestroyProjectResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aruna.api.storage.services.v1.AuthService/DestroyProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod auth_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AuthServiceServer.
    #[async_trait]
    pub trait AuthService: Send + Sync + 'static {
        /// This creates a new authorization group.option
        /// All users and collections are bundled in a authorization group.
        async fn create_project(
            &self,
            request: tonic::Request<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status>;
        ///AddUserToProject Adds a new user to a given project by its id
        async fn add_user_to_project(
            &self,
            request: tonic::Request<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status>;
        ///CreateAPIToken Creates an API token to authenticate
        async fn create_api_token(
            &self,
            request: tonic::Request<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status>;
        ///GetProjectCollections Returns all collections that belong to a certain project
        async fn get_project_collections(
            &self,
            request: tonic::Request<super::GetProjectCollectionsRequest>,
        ) -> Result<
            tonic::Response<super::GetProjectCollectionsResponse>,
            tonic::Status,
        >;
        ///GetUserCollections Returns all collections that a specified user has access to
        async fn get_user_collections(
            &self,
            request: tonic::Request<super::GetUserCollectionsRequest>,
        ) -> Result<tonic::Response<super::GetUserCollectionsResponse>, tonic::Status>;
        ///GetProject Returns the specified project
        async fn get_project(
            &self,
            request: tonic::Request<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status>;
        /// Returns all API token for a specific user
        async fn get_api_tokens(
            &self,
            request: tonic::Request<super::GetApiTokensRequest>,
        ) -> Result<tonic::Response<super::GetApiTokensResponse>, tonic::Status>;
        ///DeleteAPITokenRequest Deletes the specified API Token
        async fn delete_api_token(
            &self,
            request: tonic::Request<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status>;
        /// This will destroy the project and all its associated data.
        /// including users, collections, and API tokens and all data associated with them.
        async fn destroy_project(
            &self,
            request: tonic::Request<super::DestroyProjectRequest>,
        ) -> Result<tonic::Response<super::DestroyProjectResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AuthServiceServer<T: AuthService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AuthService> AuthServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthServiceServer<T>
    where
        T: AuthService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.AuthService/CreateProject" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProjectSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::CreateProjectRequest>
                    for CreateProjectSvc<T> {
                        type Response = super::CreateProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/AddUserToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserToProjectSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::AddUserToProjectRequest>
                    for AddUserToProjectSvc<T> {
                        type Response = super::AddUserToProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddUserToProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_user_to_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddUserToProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/CreateAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAPITokenSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::CreateApiTokenRequest>
                    for CreateAPITokenSvc<T> {
                        type Response = super::CreateApiTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_api_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAPITokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/GetProjectCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectCollectionsSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::GetProjectCollectionsRequest>
                    for GetProjectCollectionsSvc<T> {
                        type Response = super::GetProjectCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProjectCollectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_project_collections(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectCollectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/GetUserCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserCollectionsSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::GetUserCollectionsRequest>
                    for GetUserCollectionsSvc<T> {
                        type Response = super::GetUserCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserCollectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_collections(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserCollectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/GetProject" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::GetProjectRequest>
                    for GetProjectSvc<T> {
                        type Response = super::GetProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_project(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/GetAPITokens" => {
                    #[allow(non_camel_case_types)]
                    struct GetAPITokensSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::GetApiTokensRequest>
                    for GetAPITokensSvc<T> {
                        type Response = super::GetApiTokensResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApiTokensRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_api_tokens(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAPITokensSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/DeleteAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAPITokenSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::DeleteApiTokenRequest>
                    for DeleteAPITokenSvc<T> {
                        type Response = super::DeleteApiTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_api_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAPITokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.AuthService/DestroyProject" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyProjectSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::DestroyProjectRequest>
                    for DestroyProjectSvc<T> {
                        type Response = super::DestroyProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).destroy_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DestroyProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AuthService> Clone for AuthServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AuthService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthService> tonic::transport::NamedService for AuthServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.AuthService";
    }
}