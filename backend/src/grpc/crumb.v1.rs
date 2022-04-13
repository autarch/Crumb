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
/// Generated server implementations.
pub mod crumb_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CrumbServer.
    #[async_trait]
    pub trait Crumb: Send + Sync + 'static {
        ///Server streaming response type for the GetArtists method.
        type GetArtistsStream: futures_core::Stream<
                Item = Result<super::ArtistListItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_artists(
            &self,
            request: tonic::Request<super::GetArtistsRequest>,
        ) -> Result<tonic::Response<Self::GetArtistsStream>, tonic::Status>;
        async fn get_artist(
            &self,
            request: tonic::Request<super::GetArtistRequest>,
        ) -> Result<tonic::Response<super::GetArtistResponse>, tonic::Status>;
        ///Server streaming response type for the GetReleasesForArtist method.
        type GetReleasesForArtistStream: futures_core::Stream<
                Item = Result<super::ReleaseListItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_releases_for_artist(
            &self,
            request: tonic::Request<super::GetReleasesForArtistRequest>,
        ) -> Result<tonic::Response<Self::GetReleasesForArtistStream>, tonic::Status>;
        async fn get_release(
            &self,
            request: tonic::Request<super::GetReleaseRequest>,
        ) -> Result<tonic::Response<super::GetReleaseResponse>, tonic::Status>;
        ///Server streaming response type for the GetTracksForRelease method.
        type GetTracksForReleaseStream: futures_core::Stream<
                Item = Result<super::ReleaseTrack, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_tracks_for_release(
            &self,
            request: tonic::Request<super::GetTracksForReleaseRequest>,
        ) -> Result<tonic::Response<Self::GetTracksForReleaseStream>, tonic::Status>;
        ///Server streaming response type for the GetQueue method.
        type GetQueueStream: futures_core::Stream<
                Item = Result<super::QueueItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_queue(
            &self,
            request: tonic::Request<super::GetQueueRequest>,
        ) -> Result<tonic::Response<Self::GetQueueStream>, tonic::Status>;
        ///Server streaming response type for the AddToQueue method.
        type AddToQueueStream: futures_core::Stream<
                Item = Result<super::QueueItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn add_to_queue(
            &self,
            request: tonic::Request<super::AddToQueueRequest>,
        ) -> Result<tonic::Response<Self::AddToQueueStream>, tonic::Status>;
        ///Server streaming response type for the ReplaceQueue method.
        type ReplaceQueueStream: futures_core::Stream<
                Item = Result<super::QueueItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn replace_queue(
            &self,
            request: tonic::Request<super::ReplaceQueueRequest>,
        ) -> Result<tonic::Response<Self::ReplaceQueueStream>, tonic::Status>;
        ///Server streaming response type for the RemoveFromQueue method.
        type RemoveFromQueueStream: futures_core::Stream<
                Item = Result<super::QueueItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn remove_from_queue(
            &self,
            request: tonic::Request<super::RemoveFromQueueRequest>,
        ) -> Result<tonic::Response<Self::RemoveFromQueueStream>, tonic::Status>;
        ///Server streaming response type for the MoveQueueForward method.
        type MoveQueueForwardStream: futures_core::Stream<
                Item = Result<super::QueueItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn move_queue_forward(
            &self,
            request: tonic::Request<super::MoveQueueForwardRequest>,
        ) -> Result<tonic::Response<Self::MoveQueueForwardStream>, tonic::Status>;
        ///Server streaming response type for the MoveQueueBackward method.
        type MoveQueueBackwardStream: futures_core::Stream<
                Item = Result<super::QueueItem, tonic::Status>,
            >
            + Send
            + 'static;
        async fn move_queue_backward(
            &self,
            request: tonic::Request<super::MoveQueueBackwardRequest>,
        ) -> Result<tonic::Response<Self::MoveQueueBackwardStream>, tonic::Status>;
        async fn like_track(
            &self,
            request: tonic::Request<super::LikeTrackRequest>,
        ) -> Result<tonic::Response<super::LikeOrDislikeTrackResponse>, tonic::Status>;
        async fn dislike_track(
            &self,
            request: tonic::Request<super::DislikeTrackRequest>,
        ) -> Result<tonic::Response<super::LikeOrDislikeTrackResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CrumbServer<T: Crumb> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Crumb> CrumbServer<T> {
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
        /// Enable decompressing requests with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        /// Compress responses with `gzip`, if the client supports it.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CrumbServer<T>
    where
        T: Crumb,
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
                "/crumb.v1.Crumb/GetArtists" => {
                    #[allow(non_camel_case_types)]
                    struct GetArtistsSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<super::GetArtistsRequest>
                    for GetArtistsSvc<T> {
                        type Response = super::ArtistListItem;
                        type ResponseStream = T::GetArtistsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetArtistsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_artists(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetArtistsSvc(inner);
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
                "/crumb.v1.Crumb/GetArtist" => {
                    #[allow(non_camel_case_types)]
                    struct GetArtistSvc<T: Crumb>(pub Arc<T>);
                    impl<T: Crumb> tonic::server::UnaryService<super::GetArtistRequest>
                    for GetArtistSvc<T> {
                        type Response = super::GetArtistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetArtistRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_artist(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetArtistSvc(inner);
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
                "/crumb.v1.Crumb/GetReleasesForArtist" => {
                    #[allow(non_camel_case_types)]
                    struct GetReleasesForArtistSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<
                        super::GetReleasesForArtistRequest,
                    > for GetReleasesForArtistSvc<T> {
                        type Response = super::ReleaseListItem;
                        type ResponseStream = T::GetReleasesForArtistStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReleasesForArtistRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_releases_for_artist(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReleasesForArtistSvc(inner);
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
                "/crumb.v1.Crumb/GetRelease" => {
                    #[allow(non_camel_case_types)]
                    struct GetReleaseSvc<T: Crumb>(pub Arc<T>);
                    impl<T: Crumb> tonic::server::UnaryService<super::GetReleaseRequest>
                    for GetReleaseSvc<T> {
                        type Response = super::GetReleaseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReleaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_release(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReleaseSvc(inner);
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
                "/crumb.v1.Crumb/GetTracksForRelease" => {
                    #[allow(non_camel_case_types)]
                    struct GetTracksForReleaseSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<
                        super::GetTracksForReleaseRequest,
                    > for GetTracksForReleaseSvc<T> {
                        type Response = super::ReleaseTrack;
                        type ResponseStream = T::GetTracksForReleaseStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTracksForReleaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_tracks_for_release(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTracksForReleaseSvc(inner);
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
                "/crumb.v1.Crumb/GetQueue" => {
                    #[allow(non_camel_case_types)]
                    struct GetQueueSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<super::GetQueueRequest>
                    for GetQueueSvc<T> {
                        type Response = super::QueueItem;
                        type ResponseStream = T::GetQueueStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_queue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetQueueSvc(inner);
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
                "/crumb.v1.Crumb/AddToQueue" => {
                    #[allow(non_camel_case_types)]
                    struct AddToQueueSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<super::AddToQueueRequest>
                    for AddToQueueSvc<T> {
                        type Response = super::QueueItem;
                        type ResponseStream = T::AddToQueueStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddToQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_to_queue(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddToQueueSvc(inner);
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
                "/crumb.v1.Crumb/ReplaceQueue" => {
                    #[allow(non_camel_case_types)]
                    struct ReplaceQueueSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<super::ReplaceQueueRequest>
                    for ReplaceQueueSvc<T> {
                        type Response = super::QueueItem;
                        type ResponseStream = T::ReplaceQueueStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReplaceQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).replace_queue(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReplaceQueueSvc(inner);
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
                "/crumb.v1.Crumb/RemoveFromQueue" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFromQueueSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<
                        super::RemoveFromQueueRequest,
                    > for RemoveFromQueueSvc<T> {
                        type Response = super::QueueItem;
                        type ResponseStream = T::RemoveFromQueueStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveFromQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_from_queue(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFromQueueSvc(inner);
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
                "/crumb.v1.Crumb/MoveQueueForward" => {
                    #[allow(non_camel_case_types)]
                    struct MoveQueueForwardSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<
                        super::MoveQueueForwardRequest,
                    > for MoveQueueForwardSvc<T> {
                        type Response = super::QueueItem;
                        type ResponseStream = T::MoveQueueForwardStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveQueueForwardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).move_queue_forward(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveQueueForwardSvc(inner);
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
                "/crumb.v1.Crumb/MoveQueueBackward" => {
                    #[allow(non_camel_case_types)]
                    struct MoveQueueBackwardSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::ServerStreamingService<
                        super::MoveQueueBackwardRequest,
                    > for MoveQueueBackwardSvc<T> {
                        type Response = super::QueueItem;
                        type ResponseStream = T::MoveQueueBackwardStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveQueueBackwardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).move_queue_backward(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveQueueBackwardSvc(inner);
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
                "/crumb.v1.Crumb/LikeTrack" => {
                    #[allow(non_camel_case_types)]
                    struct LikeTrackSvc<T: Crumb>(pub Arc<T>);
                    impl<T: Crumb> tonic::server::UnaryService<super::LikeTrackRequest>
                    for LikeTrackSvc<T> {
                        type Response = super::LikeOrDislikeTrackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LikeTrackRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).like_track(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LikeTrackSvc(inner);
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
                "/crumb.v1.Crumb/DislikeTrack" => {
                    #[allow(non_camel_case_types)]
                    struct DislikeTrackSvc<T: Crumb>(pub Arc<T>);
                    impl<
                        T: Crumb,
                    > tonic::server::UnaryService<super::DislikeTrackRequest>
                    for DislikeTrackSvc<T> {
                        type Response = super::LikeOrDislikeTrackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DislikeTrackRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).dislike_track(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DislikeTrackSvc(inner);
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
    impl<T: Crumb> Clone for CrumbServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Crumb> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Crumb> tonic::transport::NamedService for CrumbServer<T> {
        const NAME: &'static str = "crumb.v1.Crumb";
    }
}
