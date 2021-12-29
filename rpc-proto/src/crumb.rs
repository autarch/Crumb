#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtistListItem {
    #[prost(string, tag = "1")]
    pub artist_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub sortable_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub translated_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub translated_sortable_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub transcribed_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub transcribed_sortable_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, tag = "8")]
    pub release_count: u32,
    #[prost(uint32, tag = "9")]
    pub track_count: u32,
    #[prost(string, optional, tag = "10")]
    pub album_cover_uri: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseListItem {
    #[prost(string, tag = "1")]
    pub release_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub primary_artist_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub transcribed_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub translated_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "7")]
    pub release_year: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub release_month: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub release_day: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub original_year: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub original_month: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub original_day: ::core::option::Option<u32>,
    #[prost(uint32, tag = "13")]
    pub track_count: u32,
    #[prost(string, optional, tag = "14")]
    pub album_cover_uri: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Release {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<ReleaseListItem>,
    #[prost(message, repeated, tag = "2")]
    pub tracks: ::prost::alloc::vec::Vec<Track>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Track {
    #[prost(string, tag = "1")]
    pub track_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub primary_artist_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub transcribed_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub translated_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub length: ::core::option::Option<u32>,
    #[prost(string, tag = "7")]
    pub track_uri: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListArtistsParams {}
#[doc = r" Generated client implementations."]
pub mod crumb_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CrumbClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CrumbClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CrumbClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
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
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CrumbClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn list_artists(
            &mut self,
            request: impl tonic::IntoRequest<super::ListArtistsParams>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ArtistListItem>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/crumb.Crumb/ListArtists");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod crumb_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CrumbServer."]
    #[async_trait]
    pub trait Crumb: Send + Sync + 'static {
        #[doc = "Server streaming response type for the ListArtists method."]
        type ListArtistsStream: futures_core::Stream<Item = Result<super::ArtistListItem, tonic::Status>>
            + Send
            + 'static;
        async fn list_artists(
            &self,
            request: tonic::Request<super::ListArtistsParams>,
        ) -> Result<tonic::Response<Self::ListArtistsStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CrumbServer<T: Crumb> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Crumb> CrumbServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CrumbServer<T>
    where
        T: Crumb,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/crumb.Crumb/ListArtists" => {
                    #[allow(non_camel_case_types)]
                    struct ListArtistsSvc<T: Crumb>(pub Arc<T>);
                    impl<T: Crumb> tonic::server::ServerStreamingService<super::ListArtistsParams>
                        for ListArtistsSvc<T>
                    {
                        type Response = super::ArtistListItem;
                        type ResponseStream = T::ListArtistsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListArtistsParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_artists(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListArtistsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
        const NAME: &'static str = "crumb.Crumb";
    }
}
