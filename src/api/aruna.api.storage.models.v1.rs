/// A Project is a list of collections with associated users
/// This is used to manage access to multiple collections at the same time
/// Each Collection can only be in one Project at a time
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Project {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub user_permissions: ::prost::alloc::vec::Vec<ProjectPermission>,
    #[prost(string, repeated, tag="4")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectOverview {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The actual token, this will be redacted on every get request
    #[prost(string, tag="3")]
    pub token: ::prost::alloc::string::String,
    #[prost(enumeration="TokenType", tag="4")]
    pub token_type: i32,
    #[prost(message, optional, tag="5")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub expires_on: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag="7")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="Permission", tag="8")]
    pub permission: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectPermission {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration="Permission", tag="2")]
    pub permission: i32,
    #[prost(message, repeated, tag="3")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    #[prost(enumeration="Permission", tag="1")]
    pub permission: i32,
    #[prost(enumeration="PermType", tag="2")]
    pub perm_type: i32,
    /// Can be userid, tokenid or anonymous id depending on perm_type
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// Userid of the user who created the authorization
    #[prost(string, tag="4")]
    pub created_by: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Permission {
    Unspecified = 0,
    /// No permissions granted, used for users that are in the project but have no default permissions
    None = 1,
    /// Read only
    Read = 2,
    /// Append objects to the collection cannot modify existing objects
    Append = 3,
    /// Can Read/Append/Modify objects in the collection that owns the object / Create new collections
    Modify = 4,
    /// Can modify the collections itself and permanently delete owned objects / move ownership of objects
    Admin = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PermType {
    Unspecified = 0,
    /// Regular OAuth users
    User = 1,
    /// Anonymous users without an OAuth token
    Anonymous = 2,
    /// Access token on behalf of a user
    Token = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TokenType {
    Unspecified = 0,
    Personal = 1,
    Scoped = 2,
}
/// A key value pair for hooks and labels
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelOntology {
    /// These are the keys for labels that are required for the collection
    /// Adding an Object without these keys will result in an error
    /// Defaults to empty string if not specified
    #[prost(string, repeated, tag="1")]
    pub required_label_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This is a message to uniquely identify specific Objects
/// IDs by itself are not unique
/// Only the combination of ID and revision number is unique
/// A revision number of -1 corresponds to a "latest" tag
/// This will automatically update itself
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevId {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub revision: i64,
}
/// S3 location(s) for an object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// default location
    #[prost(message, optional, tag="1")]
    pub default: ::core::option::Option<ObjectLocation>,
    /// list of locations
    #[prost(message, repeated, tag="2")]
    pub locations: ::prost::alloc::vec::Vec<ObjectLocation>,
}
/// A location in S3
/// This can be either a location_id as a description e.g. "giessen" or a specific S3 location
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectLocation {
    #[prost(oneof="object_location::Location", tags="1, 2")]
    pub location: ::core::option::Option<object_location::Location>,
}
/// Nested message and enum types in `ObjectLocation`.
pub mod object_location {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        #[prost(string, tag="1")]
        LocationId(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        S3Location(super::S3Location),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S3Location {
    /// Bucket in S3
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Key in S3
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    /// Object storage endpoint
    #[prost(string, tag="3")]
    pub url: ::prost::alloc::string::String,
}
/// Stats for a set of objects
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    #[prost(int64, tag="1")]
    pub count: i64,
    #[prost(int64, tag="2")]
    pub acc_size: i64,
}
/// Stats for a collection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionStats {
    #[prost(message, optional, tag="1")]
    pub object_stats: ::core::option::Option<Stats>,
    #[prost(int64, tag="2")]
    pub object_group_count: i64,
    #[prost(message, optional, tag="3")]
    pub specification_stats: ::core::option::Option<Stats>,
    #[prost(message, optional, tag="6")]
    pub last_date_updated: ::core::option::Option<::prost_types::Timestamp>,
}
/// Stats for an object group
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupStats {
    #[prost(message, optional, tag="1")]
    pub total_stats: ::core::option::Option<Stats>,
    #[prost(message, optional, tag="2")]
    pub object_stats: ::core::option::Option<Stats>,
    #[prost(message, optional, tag="3")]
    pub meta_object_stats: ::core::option::Option<Stats>,
}
/// Semver version -> Alpha Beta release are not supported -> Use "latest" for mutable collections that are in development
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(int32, tag="1")]
    pub major: i32,
    #[prost(int32, tag="2")]
    pub minor: i32,
    #[prost(int32, tag="3")]
    pub patch: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    #[prost(enumeration="Hashalgorithm", tag="1")]
    pub alg: i32,
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
}
/// Origin of the object -> To be GDPA compliant
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Origin {
    #[prost(enumeration="OriginType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
// RULES for Objects:
// 1.  Each object is "owned" by exactly one collection
// 2.  Objects can be "borrowed" to multiple other collections
// 3.  Objects are immutable, updating an object will create a new object with increased revision number
//     only people with modify permissions in the owner collection can update an object
// 3.1 Special cases: 
//     Hooks: Can be added/removed and modified without changing the object revision number
//     Labels: Can be added without changing the object revision number, removing or modifying labels WILL change the object revision number (append only)
//     auto_update: Can be added/removed without changing the object revision number and is collection specific
// 4.  Objects can only be permanently deleted by a person with admin rights on the owner collection

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    ///ObjectID -> This is not unique across the database -> Composite key with revision
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Filename: Name of the original file e.g.: mydata.json
    #[prost(string, tag="2")]
    pub filename: ::prost::alloc::string::String,
    /// Labels to additionally describe the object
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    /// Hooks to be executed on the object
    #[prost(message, repeated, tag="5")]
    pub hooks: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, optional, tag="6")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// Each object is owned by exactly one collection
    #[prost(string, tag="7")]
    pub collection_id: ::prost::alloc::string::String,
    /// Location of the data
    #[prost(message, optional, tag="8")]
    pub location: ::core::option::Option<Location>,
    /// Lenght of the stored dataset
    #[prost(int64, tag="9")]
    pub content_len: i64,
    #[prost(enumeration="Status", tag="10")]
    pub status: i32,
    /// Origin of the object
    #[prost(message, optional, tag="11")]
    pub origin: ::core::option::Option<Origin>,
    /// Confidentiality of the object
    #[prost(enumeration="DataClass", tag="12")]
    pub data_class: i32,
    /// MD5 hash of the data
    #[prost(message, optional, tag="13")]
    pub hash: ::core::option::Option<Hash>,
    /// Increasing revion number for each update -> This is used in the database
    #[prost(int64, tag="14")]
    pub rev_number: i64,
    /// Is this the latest version of the object?
    #[prost(bool, tag="15")]
    pub latest: bool,
    /// This is a collection specific attribute
    /// Must be false if collection is immutable
    ///
    /// If true, the object will be updated automatically when the data is changed
    #[prost(bool, tag="16")]
    pub auto_update: bool,
}
/// Multiple Objects
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Objects {
    #[prost(message, repeated, tag="1")]
    pub objects: ::prost::alloc::vec::Vec<Object>,
}
/// ObjectGroups are optional and can be used to group objects in a collection together
/// They need to refer to objects in the same collection
/// Objectgroups can be changed if the collection is mutable
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroup {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Exactly one collection must be specified
    #[prost(string, tag="4")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, repeated, tag="7")]
    pub hooks: ::prost::alloc::vec::Vec<KeyValue>,
    /// Must be in collection objects
    #[prost(message, repeated, tag="8")]
    pub objects: ::prost::alloc::vec::Vec<Object>,
    /// Must be in collection objects
    #[prost(message, repeated, tag="9")]
    pub meta_objects: ::prost::alloc::vec::Vec<Object>,
    #[prost(message, optional, tag="10")]
    pub stats: ::core::option::Option<ObjectGroupStats>,
    #[prost(int64, tag="11")]
    pub rev_number: i64,
}
/// Multiple ObjectGroups
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroups {
    #[prost(message, repeated, tag="1")]
    pub object_groups: ::prost::alloc::vec::Vec<ObjectGroup>,
}
/// This is a representation of the ObjectGroup without the recursive nature of object references
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupOverview {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Exactly one collection must be specified
    #[prost(string, tag="4")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, repeated, tag="7")]
    pub hooks: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, optional, tag="8")]
    pub stats: ::core::option::Option<ObjectGroupStats>,
    #[prost(int64, tag="9")]
    pub rev_number: i64,
}
/// Multiple ObjectGroupOverviews
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupOverviews {
    #[prost(message, repeated, tag="1")]
    pub object_group_overviews: ::prost::alloc::vec::Vec<ObjectGroupOverview>,
}
/// This is a representation of the ObjectGroup with only ObjectIDs instead of full objects
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupWithId {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Exactly one collection must be specified
    #[prost(string, tag="4")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, repeated, tag="7")]
    pub hooks: ::prost::alloc::vec::Vec<KeyValue>,
    /// Must be in collection objects
    #[prost(message, repeated, tag="8")]
    pub object_ids: ::prost::alloc::vec::Vec<RevId>,
    /// Must be in collection objects
    #[prost(message, repeated, tag="9")]
    pub meta_object_ids: ::prost::alloc::vec::Vec<RevId>,
    #[prost(message, optional, tag="10")]
    pub stats: ::core::option::Option<ObjectGroupStats>,
    #[prost(int64, tag="11")]
    pub rev_number: i64,
}
/// Multiple ObjectGroupWithIDs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupWithIDs {
    #[prost(message, repeated, tag="1")]
    pub object_group_with_ids: ::prost::alloc::vec::Vec<ObjectGroupWithId>,
}
// RULES for Collections:
// 1. Each object is "owned" by exactly one collection
// 2. Objects can be in multiple collections and must be in the owner collection
// 3. Collections are either mutable with Version.latest == true or immutable with a fixed version number
// 3.1 If a collection gets a fixed version a copy is created with all "latest" objects dereferenced to their respective revisions
// 3.2 Modifying an immutable collection will create a new copy of the collection with a new version number
// 4. Collections can be created by any user, but only the owner can modify or delete them

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Should be unique in authgroup
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, repeated, tag="5")]
    pub hooks: ::prost::alloc::vec::Vec<KeyValue>,
    /// Ontology for labels
    #[prost(message, optional, tag="6")]
    pub label_ontology: ::core::option::Option<LabelOntology>,
    #[prost(message, optional, tag="7")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="8")]
    pub objects: ::prost::alloc::vec::Vec<Object>,
    #[prost(message, repeated, tag="9")]
    pub specifications: ::prost::alloc::vec::Vec<Object>,
    #[prost(message, repeated, tag="10")]
    pub object_groups: ::prost::alloc::vec::Vec<ObjectGroup>,
    #[prost(message, repeated, tag="11")]
    pub authorization: ::prost::alloc::vec::Vec<Authorization>,
    #[prost(message, optional, tag="14")]
    pub stats: ::core::option::Option<CollectionStats>,
    #[prost(bool, tag="15")]
    pub is_public: bool,
    #[prost(oneof="collection::Version", tags="12, 13")]
    pub version: ::core::option::Option<collection::Version>,
}
/// Nested message and enum types in `Collection`.
pub mod collection {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="12")]
        SemanticVersion(super::Version),
        #[prost(bool, tag="13")]
        Latest(bool),
    }
}
/// Multiple Collections
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collections {
    #[prost(message, repeated, tag="1")]
    pub collections: ::prost::alloc::vec::Vec<Collection>,
}
/// This is a representation of the Collection without the recursive nature of objectreferences
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionOverview {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, repeated, tag="5")]
    pub hooks: ::prost::alloc::vec::Vec<KeyValue>,
    /// Ontology for labels
    #[prost(message, optional, tag="6")]
    pub label_ontology: ::core::option::Option<LabelOntology>,
    #[prost(message, optional, tag="7")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="11")]
    pub authorization: ::prost::alloc::vec::Vec<Authorization>,
    #[prost(message, optional, tag="14")]
    pub stats: ::core::option::Option<CollectionStats>,
    #[prost(bool, tag="15")]
    pub is_public: bool,
    #[prost(oneof="collection_overview::Version", tags="12, 13")]
    pub version: ::core::option::Option<collection_overview::Version>,
}
/// Nested message and enum types in `CollectionOverview`.
pub mod collection_overview {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="12")]
        SemanticVersion(super::Version),
        #[prost(bool, tag="13")]
        Latest(bool),
    }
}
/// Multiple CollectionOverviews
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionOverviews {
    #[prost(message, repeated, tag="1")]
    pub collection_overviews: ::prost::alloc::vec::Vec<CollectionOverview>,
}
/// This is a representation of the Collection with only Resource RevisionIDs instead of full objects
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionWithId {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(message, repeated, tag="5")]
    pub hooks: ::prost::alloc::vec::Vec<KeyValue>,
    /// Ontology for labels
    #[prost(message, optional, tag="6")]
    pub label_ontology: ::core::option::Option<LabelOntology>,
    #[prost(message, optional, tag="7")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="8")]
    pub objects: ::prost::alloc::vec::Vec<RevId>,
    #[prost(message, repeated, tag="9")]
    pub specifications: ::prost::alloc::vec::Vec<RevId>,
    #[prost(message, repeated, tag="10")]
    pub object_groups: ::prost::alloc::vec::Vec<RevId>,
    #[prost(message, repeated, tag="11")]
    pub authorization: ::prost::alloc::vec::Vec<Authorization>,
    #[prost(message, optional, tag="14")]
    pub stats: ::core::option::Option<CollectionStats>,
    #[prost(bool, tag="15")]
    pub is_public: bool,
    #[prost(oneof="collection_with_id::Version", tags="12, 13")]
    pub version: ::core::option::Option<collection_with_id::Version>,
}
/// Nested message and enum types in `CollectionWithID`.
pub mod collection_with_id {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="12")]
        SemanticVersion(super::Version),
        #[prost(bool, tag="13")]
        Latest(bool),
    }
}
/// Multiple CollectionWithIDs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionWithIDs {
    #[prost(message, repeated, tag="1")]
    pub collection_with_ids: ::prost::alloc::vec::Vec<CollectionWithId>,
}
/// An arbitrary status for Objects
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Unspecified = 0,
    Initializing = 1,
    Available = 2,
    Unavailable = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Hashalgorithm {
    Unspecified = 0,
    Md5 = 1,
    Sha1 = 2,
    Sha256 = 3,
    Sha512 = 4,
    Murmur3a32 = 5,
    Xxhash32 = 6,
}
/// Specifies the Origin of the object
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OriginType {
    Unspecified = 0,
    /// User uploaded the object
    User = 1,
    /// Object was cloned from another object
    Objclone = 2,
}
/// Dataclass defines the confidentiality of the object
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataClass {
    Unspecified = 0,
    Public = 1,
    Private = 2,
    Confidential = 3,
    Protected = 4,
}
/// This file contains parameters for queries that return a list of resources.
/// The results are paginated.
/// The page request specifies the page size and last_id.
/// If page_size is not specified, it defaults to 20.
/// If page_size is -1, it returns all objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageRequest {
    /// This is the last ID of the previous returned request
    #[prost(string, tag="1")]
    pub last_uuid: ::prost::alloc::string::String,
    /// Default to 20, -1 for all
    #[prost(uint64, tag="2")]
    pub page_size: u64,
}
/// LabelFilter is used to filter resources by labels.
/// The labels are specified as a map of key-value pairs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelFilter {
    #[prost(message, repeated, tag="1")]
    pub labels: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(enumeration="LabelQueryType", tag="2")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceIdList {
    #[prost(string, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This is a combined query for either a list of resource IDs or filtered by Label
/// Can be expanded in the future to allow for more complex queries
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelOrIdQuery {
    #[prost(oneof="label_or_id_query::Query", tags="1, 2")]
    pub query: ::core::option::Option<label_or_id_query::Query>,
}
/// Nested message and enum types in `LabelOrIDQuery`.
pub mod label_or_id_query {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        Labels(super::LabelFilter),
        #[prost(message, tag="2")]
        Ids(super::ResourceIdList),
    }
}
/// This defines the behaviour of the Query how to combine multiple Labels
/// The default Query Type is: AND so each Label must be present in the result
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelQueryType {
    Unspecified = 0,
    /// A AND B
    And = 1,
    /// A OR B including A AND B, This is similar to Any
    Or = 2,
    /// Either A OR B but not both
    Xor = 3,
    /// Return all objects with a specific Key independent of value
    Key = 4,
}
/// OutputFormat is an enum for GET queries that specifies how the Response should be structured
/// This is necessary because Collections and ObjectGroups can contain large number of objects
/// The default behaviour is "OVERVIEW" this will only return statistics about the collection or objectgroup 
/// but no specific IDs or additional information
/// the "WITH_IDS" tag returns only lists of RevIDs
/// If "FULL" is specified the complete recursive list of objects with all labels and meta-information will be returned
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputFormat {
    /// Unspecified enum will default to overview
    Unspecified = 0,
    /// Only give an overview of the resources
    Overview = 1,
    /// Return only a list of RevIDs
    WithIds = 2,
    /// Return the full list of resources -> This may have slow performance
    Full = 3,
}