#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(uint64, tag = "1")]
    pub seconds: u64,
    #[prost(uint32, tag = "2")]
    pub nanos: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePostRequest {
    #[prost(message, optional, tag = "1")]
    pub post: ::std::option::Option<Post>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadPostRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePostRequest {
    #[prost(message, optional, tag = "1")]
    pub post: ::std::option::Option<Post>,
    #[prost(message, optional, tag = "2")]
    pub mask: ::std::option::Option<PostMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePostRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostRequest {
    #[prost(uint32, tag = "1")]
    pub page_size: u32,
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::std::vec::Vec<Post>,
    #[prost(message, repeated, tag = "2")]
    pub around: ::std::vec::Vec<ListPage>,
    #[prost(message, optional, tag = "3")]
    pub previous: ::std::option::Option<ListPage>,
    #[prost(message, optional, tag = "4")]
    pub next: ::std::option::Option<ListPage>,
    #[prost(message, optional, tag = "5")]
    pub first: ::std::option::Option<ListPage>,
    #[prost(message, optional, tag = "6")]
    pub last: ::std::option::Option<ListPage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPage {
    #[prost(string, tag = "1")]
    pub page_token: std::string::String,
    #[prost(uint32, tag = "2")]
    pub page_number: u32,
    #[prost(bool, tag = "3")]
    pub current: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostMask {
    #[prost(string, repeated, tag = "1")]
    pub field: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub body: std::string::String,
    #[prost(message, repeated, tag = "3")]
    pub tags: ::std::vec::Vec<Tag>,
    /// Output only field
    #[prost(message, optional, tag = "4")]
    pub created_at: ::std::option::Option<Timestamp>,
    /// Output only field
    #[prost(message, optional, tag = "5")]
    pub updated_at: ::std::option::Option<Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub url: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod post_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PostServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PostServiceClient<tonic::transport::Channel> {
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
    impl<T> PostServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn create_post(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePostRequest>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blog.PostService/CreatePost");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read_post(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadPostRequest>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blog.PostService/ReadPost");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_post(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePostRequest>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blog.PostService/UpdatePost");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_post(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePostRequest>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blog.PostService/DeletePost");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_post(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPostRequest>,
        ) -> Result<tonic::Response<super::ListPostResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blog.PostService/ListPost");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PostServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PostServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PostServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod post_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PostServiceServer."]
    #[async_trait]
    pub trait PostService: Send + Sync + 'static {
        async fn create_post(
            &self,
            request: tonic::Request<super::CreatePostRequest>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn read_post(
            &self,
            request: tonic::Request<super::ReadPostRequest>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn update_post(
            &self,
            request: tonic::Request<super::UpdatePostRequest>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn delete_post(
            &self,
            request: tonic::Request<super::DeletePostRequest>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
        async fn list_post(
            &self,
            request: tonic::Request<super::ListPostRequest>,
        ) -> Result<tonic::Response<super::ListPostResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PostServiceServer<T: PostService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PostService> PostServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for PostServiceServer<T>
    where
        T: PostService,
        B: HttpBody + Send + Sync + 'static,
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
                "/blog.PostService/CreatePost" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePostSvc<T: PostService>(pub Arc<T>);
                    impl<T: PostService> tonic::server::UnaryService<super::CreatePostRequest> for CreatePostSvc<T> {
                        type Response = super::Post;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreatePostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/blog.PostService/ReadPost" => {
                    #[allow(non_camel_case_types)]
                    struct ReadPostSvc<T: PostService>(pub Arc<T>);
                    impl<T: PostService> tonic::server::UnaryService<super::ReadPostRequest> for ReadPostSvc<T> {
                        type Response = super::Post;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadPostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReadPostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/blog.PostService/UpdatePost" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePostSvc<T: PostService>(pub Arc<T>);
                    impl<T: PostService> tonic::server::UnaryService<super::UpdatePostRequest> for UpdatePostSvc<T> {
                        type Response = super::Post;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdatePostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/blog.PostService/DeletePost" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePostSvc<T: PostService>(pub Arc<T>);
                    impl<T: PostService> tonic::server::UnaryService<super::DeletePostRequest> for DeletePostSvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeletePostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/blog.PostService/ListPost" => {
                    #[allow(non_camel_case_types)]
                    struct ListPostSvc<T: PostService>(pub Arc<T>);
                    impl<T: PostService> tonic::server::UnaryService<super::ListPostRequest> for ListPostSvc<T> {
                        type Response = super::ListPostResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListPostSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PostService> Clone for PostServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PostService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PostService> tonic::transport::NamedService for PostServiceServer<T> {
        const NAME: &'static str = "blog.PostService";
    }
}
