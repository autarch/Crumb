/// This is based on the Google Cloud API type in
/// <https://github.com/googleapis/googleapis/blob/master/google/rpc/.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// The status code, which should be an enum value of \[google.rpc.Code][google.rpc.Code\].
    #[prost(enumeration="Code", tag="1")]
    pub code: i32,
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// \[google.rpc.Status.details][google.rpc.Status.details\] field, or localized by the client.
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    #[prost(message, repeated, tag="3")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtistListItem {
    #[prost(string, tag="1")]
    pub artist_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub sortable_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub transcripted_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub transcripted_sortable_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub translated_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub translated_sortable_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, tag="9")]
    pub release_count: u32,
    #[prost(uint32, tag="10")]
    pub track_count: u32,
    #[prost(string, optional, tag="11")]
    pub release_cover_uri: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtistItem {
    #[prost(message, optional, tag="1")]
    pub core: ::core::option::Option<ArtistListItem>,
    #[prost(message, repeated, tag="2")]
    pub releases: ::prost::alloc::vec::Vec<ReleaseListItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseListItem {
    #[prost(string, tag="1")]
    pub release_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub primary_artist_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub display_title: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub transcripted_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub translated_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="8")]
    pub release_year: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub release_month: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub release_day: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub original_year: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub original_month: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub original_day: ::core::option::Option<u32>,
    #[prost(uint32, tag="14")]
    pub track_count: u32,
    #[prost(string, optional, tag="15")]
    pub release_cover_uri: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseTrack {
    #[prost(string, tag="1")]
    pub track_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub primary_artist_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub display_title: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub transcripted_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub translated_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="7")]
    pub length: ::core::option::Option<u32>,
    #[prost(string, tag="8")]
    pub track_audio_uri: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub release_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="10")]
    pub position: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseItem {
    #[prost(message, optional, tag="1")]
    pub core: ::core::option::Option<ReleaseListItem>,
    #[prost(message, optional, tag="2")]
    pub artist: ::core::option::Option<ArtistListItem>,
    #[prost(message, repeated, tag="3")]
    pub tracks: ::prost::alloc::vec::Vec<ReleaseTrack>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueItem {
    #[prost(message, optional, tag="1")]
    pub release_track: ::core::option::Option<ReleaseTrack>,
    #[prost(string, tag="2")]
    pub release_display_title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub release_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub release_cover_uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub artist_display_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub artist_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub queue_position: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub is_current: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArtistsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArtistRequest {
    #[prost(string, tag="1")]
    pub artist_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArtistResponse {
    #[prost(oneof="get_artist_response::ResponseEither", tags="1, 2")]
    pub response_either: ::core::option::Option<get_artist_response::ResponseEither>,
}
/// Nested message and enum types in `GetArtistResponse`.
pub mod get_artist_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseEither {
        #[prost(message, tag="1")]
        Artist(super::ArtistItem),
        #[prost(message, tag="2")]
        Error(super::Status),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReleasesForArtistRequest {
    #[prost(string, tag="1")]
    pub artist_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReleaseRequest {
    #[prost(string, tag="1")]
    pub release_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReleaseResponse {
    #[prost(oneof="get_release_response::ResponseEither", tags="1, 2")]
    pub response_either: ::core::option::Option<get_release_response::ResponseEither>,
}
/// Nested message and enum types in `GetReleaseResponse`.
pub mod get_release_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseEither {
        #[prost(message, tag="1")]
        Release(super::ReleaseItem),
        #[prost(message, tag="2")]
        Error(super::Status),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTracksForReleaseRequest {
    #[prost(string, tag="1")]
    pub release_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueueRequest {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToQueueRequest {
    #[prost(string, repeated, tag="1")]
    pub track_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromQueueRequest {
    #[prost(string, repeated, tag="1")]
    pub positions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceQueueRequest {
    #[prost(string, repeated, tag="1")]
    pub track_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveQueueForwardRequest {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveQueueBackwardRequest {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeTrackRequest {
    #[prost(string, tag="1")]
    pub track_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DislikeTrackRequest {
    #[prost(string, tag="1")]
    pub track_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeOrDislikeTrackResponse {
}
// XXX - Tonic already includes Code and Status for its own internal use. Can
// we reuse that somehow?

/// The canonical error codes for gRPC APIs.
///
///
/// Sometimes multiple error codes may apply.  Services should return
/// the most specific error code that applies.  For example, prefer
/// `OUT_OF_RANGE` over `FAILED_PRECONDITION` if both codes apply.
/// Similarly prefer `NOT_FOUND` or `ALREADY_EXISTS` over `FAILED_PRECONDITION`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Code {
    /// Not an error; returned on success
    ///
    /// HTTP Mapping: 200 OK
    Ok = 0,
    /// The operation was cancelled, typically by the caller.
    ///
    /// HTTP Mapping: 499 Client Closed Request
    Cancelled = 1,
    /// Unknown error.  For example, this error may be returned when
    /// a `Status` value received from another address space belongs to
    /// an error space that is not known in this address space.  Also
    /// errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    Unknown = 2,
    /// The client specified an invalid argument.  Note that this differs
    /// from `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments
    /// that are problematic regardless of the state of the system
    /// (e.g., a malformed file name).
    ///
    /// HTTP Mapping: 400 Bad Request
    InvalidArgument = 3,
    /// The deadline expired before the operation could complete. For operations
    /// that change the state of the system, this error may be returned
    /// even if the operation has completed successfully.  For example, a
    /// successful response from a server could have been delayed long
    /// enough for the deadline to expire.
    ///
    /// HTTP Mapping: 504 Gateway Timeout
    DeadlineExceeded = 4,
    /// Some requested entity (e.g., file or directory) was not found.
    ///
    /// Note to server developers: if a request is denied for an entire class
    /// of users, such as gradual feature rollout or undocumented whitelist,
    /// `NOT_FOUND` may be used. If a request is denied for some users within
    /// a class of users, such as user-based access control, `PERMISSION_DENIED`
    /// must be used.
    ///
    /// HTTP Mapping: 404 Not Found
    NotFound = 5,
    /// The entity that a client attempted to create (e.g., file or directory)
    /// already exists.
    ///
    /// HTTP Mapping: 409 Conflict
    AlreadyExists = 6,
    /// The caller does not have permission to execute the specified
    /// operation. `PERMISSION_DENIED` must not be used for rejections
    /// caused by exhausting some resource (use `RESOURCE_EXHAUSTED`
    /// instead for those errors). `PERMISSION_DENIED` must not be
    /// used if the caller can not be identified (use `UNAUTHENTICATED`
    /// instead for those errors). This error code does not imply the
    /// request is valid or the requested entity exists or satisfies
    /// other pre-conditions.
    ///
    /// HTTP Mapping: 403 Forbidden
    PermissionDenied = 7,
    /// The request does not have valid authentication credentials for the
    /// operation.
    ///
    /// HTTP Mapping: 401 Unauthorized
    Unauthenticated = 16,
    /// Some resource has been exhausted, perhaps a per-user quota, or
    /// perhaps the entire file system is out of space.
    ///
    /// HTTP Mapping: 429 Too Many Requests
    ResourceExhausted = 8,
    /// The operation was rejected because the system is not in a state
    /// required for the operation's execution.  For example, the directory
    /// to be deleted is non-empty, an rmdir operation is applied to
    /// a non-directory, etc.
    ///
    /// Service implementors can use the following guidelines to decide
    /// between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:
    ///  (a) Use `UNAVAILABLE` if the client can retry just the failing call.
    ///  (b) Use `ABORTED` if the client should retry at a higher level
    ///      (e.g., when a client-specified test-and-set fails, indicating the
    ///      client should restart a read-modify-write sequence).
    ///  (c) Use `FAILED_PRECONDITION` if the client should not retry until
    ///      the system state has been explicitly fixed.  E.g., if an "rmdir"
    ///      fails because the directory is non-empty, `FAILED_PRECONDITION`
    ///      should be returned since the client should not retry unless
    ///      the files are deleted from the directory.
    ///
    /// HTTP Mapping: 400 Bad Request
    FailedPrecondition = 9,
    /// The operation was aborted, typically due to a concurrency issue such as
    /// a sequencer check failure or transaction abort.
    ///
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    ///
    /// HTTP Mapping: 409 Conflict
    Aborted = 10,
    /// The operation was attempted past the valid range.  E.g., seeking or
    /// reading past end-of-file.
    ///
    /// Unlike `INVALID_ARGUMENT`, this error indicates a problem that may
    /// be fixed if the system state changes. For example, a 32-bit file
    /// system will generate `INVALID_ARGUMENT` if asked to read at an
    /// offset that is not in the range \[0,2^32-1\], but it will generate
    /// `OUT_OF_RANGE` if asked to read from an offset past the current
    /// file size.
    ///
    /// There is a fair bit of overlap between `FAILED_PRECONDITION` and
    /// `OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific
    /// error) when it applies so that callers who are iterating through
    /// a space can easily look for an `OUT_OF_RANGE` error to detect when
    /// they are done.
    ///
    /// HTTP Mapping: 400 Bad Request
    OutOfRange = 11,
    /// The operation is not implemented or is not supported/enabled in this
    /// service.
    ///
    /// HTTP Mapping: 501 Not Implemented
    Unimplemented = 12,
    /// Internal errors.  This means that some invariants expected by the
    /// underlying system have been broken.  This error code is reserved
    /// for serious errors.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    Internal = 13,
    /// The service is currently unavailable.  This is most likely a
    /// transient condition, which can be corrected by retrying with
    /// a backoff. Note that it is not always safe to retry
    /// non-idempotent operations.
    ///
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    ///
    /// HTTP Mapping: 503 Service Unavailable
    Unavailable = 14,
    /// Unrecoverable data loss or corruption.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    DataLoss = 15,
}
/// Generated client implementations.
pub mod crumb_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CrumbClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CrumbClient<T>
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
        ) -> CrumbClient<InterceptedService<T, F>>
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
            CrumbClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_artists(
            &mut self,
            request: impl tonic::IntoRequest<super::GetArtistsRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::ArtistListItem>>,
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
                "/crumb.v1.Crumb/GetArtists",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_artist(
            &mut self,
            request: impl tonic::IntoRequest<super::GetArtistRequest>,
        ) -> Result<tonic::Response<super::GetArtistResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/crumb.v1.Crumb/GetArtist");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_releases_for_artist(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReleasesForArtistRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::ReleaseListItem>>,
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
                "/crumb.v1.Crumb/GetReleasesForArtist",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_release(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReleaseRequest>,
        ) -> Result<tonic::Response<super::GetReleaseResponse>, tonic::Status> {
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
                "/crumb.v1.Crumb/GetRelease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_tracks_for_release(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTracksForReleaseRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::ReleaseTrack>>,
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
                "/crumb.v1.Crumb/GetTracksForRelease",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQueueRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::QueueItem>>,
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
            let path = http::uri::PathAndQuery::from_static("/crumb.v1.Crumb/GetQueue");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn add_to_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::AddToQueueRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::QueueItem>>,
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
                "/crumb.v1.Crumb/AddToQueue",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn replace_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceQueueRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::QueueItem>>,
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
                "/crumb.v1.Crumb/ReplaceQueue",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn remove_from_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFromQueueRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::QueueItem>>,
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
                "/crumb.v1.Crumb/RemoveFromQueue",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn move_queue_forward(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveQueueForwardRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::QueueItem>>,
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
                "/crumb.v1.Crumb/MoveQueueForward",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn move_queue_backward(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveQueueBackwardRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::QueueItem>>,
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
                "/crumb.v1.Crumb/MoveQueueBackward",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn like_track(
            &mut self,
            request: impl tonic::IntoRequest<super::LikeTrackRequest>,
        ) -> Result<tonic::Response<super::LikeOrDislikeTrackResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/crumb.v1.Crumb/LikeTrack");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn dislike_track(
            &mut self,
            request: impl tonic::IntoRequest<super::DislikeTrackRequest>,
        ) -> Result<tonic::Response<super::LikeOrDislikeTrackResponse>, tonic::Status> {
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
                "/crumb.v1.Crumb/DislikeTrack",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
