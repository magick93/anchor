
//
// This file is generated from openapi specification. Please do not modify it.
// It should be .gitignored
//
#![allow(warnings)]
#![allow(clippy::all)]

use axum::{
    body::{Body, HttpBody},
    extract::{FromRequest, FromRequestParts, Request as ERequest},
    handler::{future::IntoServiceFuture, Handler},
    http::Request,
    response::{IntoResponse, Response},
    routing::{self, on_service, MethodFilter},
    Router, BoxError,
};
use futures::{future::LocalBoxFuture, Future};
use std::{
    convert::{Infallible, TryFrom},
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
    }
}

/// Accounts router
pub struct AccountsRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> AccountsRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/accounts
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery) -> AccountV4ControllerListResponse {
    ///     todo!();
    /// }
    /// let router = AccountsRouter::default().account_v4_controller_list(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery, ...extractors) -> AccountV4ControllerListResponse {
    ///     todo!();
    /// }
    /// let router = AccountsRouter::default().account_v4_controller_list(handler);
    /// ```
    pub fn account_v4_controller_list<H, T>(mut self, handler: H) -> Self
        where
            H: AccountV4ControllerListHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/accounts", on_service(MethodFilter::GET, AccountV4ControllerListService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/accounts/{ownerAddress}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerByIdPath) -> AccountV4ControllerByIdResponse {
    ///     todo!();
    /// }
    /// let router = AccountsRouter::default().account_v4_controller_by_id(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerByIdPath, ...extractors) -> AccountV4ControllerByIdResponse {
    ///     todo!();
    /// }
    /// let router = AccountsRouter::default().account_v4_controller_by_id(handler);
    /// ```
    pub fn account_v4_controller_by_id<H, T>(mut self, handler: H) -> Self
        where
            H: AccountV4ControllerByIdHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/accounts/:ownerAddress", on_service(MethodFilter::GET, AccountV4ControllerByIdService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/accounts/counts/{ownerAddress}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4AccountsCountsGetPath) -> ApiV4AccountsCountsGetResponse {
    ///     todo!();
    /// }
    /// let router = AccountsRouter::default().api_v4_accounts_counts_get(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4AccountsCountsGetPath, ...extractors) -> ApiV4AccountsCountsGetResponse {
    ///     todo!();
    /// }
    /// let router = AccountsRouter::default().api_v4_accounts_counts_get(handler);
    /// ```
    pub fn api_v4_accounts_counts_get<H, T>(mut self, handler: H) -> Self
        where
            H: ApiV4AccountsCountsGetHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/accounts/counts/:ownerAddress", on_service(MethodFilter::GET, ApiV4AccountsCountsGetService::new(handler)));
        self
    }
}

impl<S> Default for AccountsRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<AccountsRouter<S>> for Router<S> {
    fn from(r: AccountsRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/accounts handler
pub trait AccountV4ControllerListHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> AccountV4ControllerListHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::AccountV4ControllerListResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerListPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::AccountV4ControllerListQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::AccountV4ControllerListResponse = res.into();

            response.into()
        })
    }
}

macro_rules! account_v4_controller_list_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> AccountV4ControllerListHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::AccountV4ControllerListResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerListPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::AccountV4ControllerListQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::AccountV4ControllerListResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(account_v4_controller_list_handler);

/// GET /api/v4/{network}/accounts service
struct AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( AccountV4ControllerListHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/accounts/{ownerAddress} handler
pub trait AccountV4ControllerByIdHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> AccountV4ControllerByIdHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::AccountV4ControllerByIdPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::AccountV4ControllerByIdResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerByIdPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::AccountV4ControllerByIdResponse = res.into();

            response.into()
        })
    }
}

macro_rules! account_v4_controller_by_id_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> AccountV4ControllerByIdHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::AccountV4ControllerByIdPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::AccountV4ControllerByIdResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerByIdPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::AccountV4ControllerByIdResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(account_v4_controller_by_id_handler);

/// GET /api/v4/{network}/accounts/{ownerAddress} service
struct AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( AccountV4ControllerByIdHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/accounts/counts/{ownerAddress} handler
pub trait ApiV4AccountsCountsGetHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ApiV4AccountsCountsGetHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ApiV4AccountsCountsGetPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ApiV4AccountsCountsGetResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ApiV4AccountsCountsGetPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ApiV4AccountsCountsGetResponse = res.into();

            response.into()
        })
    }
}

macro_rules! api_v4_accounts_counts_get_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ApiV4AccountsCountsGetHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ApiV4AccountsCountsGetPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ApiV4AccountsCountsGetResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ApiV4AccountsCountsGetPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ApiV4AccountsCountsGetResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(api_v4_accounts_counts_get_handler);

/// GET /api/v4/{network}/accounts/counts/{ownerAddress} service
struct ApiV4AccountsCountsGetService<H, T, S>
where
    H: ApiV4AccountsCountsGetHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ApiV4AccountsCountsGetService<H, T, S>
where
    H: ApiV4AccountsCountsGetHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ApiV4AccountsCountsGetService<H, T, S>
where
    H: ApiV4AccountsCountsGetHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ApiV4AccountsCountsGetService<H, T, S>
where
    H: ApiV4AccountsCountsGetHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ApiV4AccountsCountsGetHandler::call(handler, req, state).await)
        })
    }
}



/// Clusters router
pub struct ClustersRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> ClustersRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/clusters/count
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerCountPath) -> ClusterV4ControllerCountResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_count(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerCountPath, ...extractors) -> ClusterV4ControllerCountResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_count(handler);
    /// ```
    pub fn cluster_v4_controller_count<H, T>(mut self, handler: H) -> Self
        where
            H: ClusterV4ControllerCountHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/clusters/count", on_service(MethodFilter::GET, ClusterV4ControllerCountService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/clusters
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerListPath,super::endpoint::ClusterV4ControllerListQuery) -> ClusterV4ControllerListResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_list(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerListPath,super::endpoint::ClusterV4ControllerListQuery, ...extractors) -> ClusterV4ControllerListResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_list(handler);
    /// ```
    pub fn cluster_v4_controller_list<H, T>(mut self, handler: H) -> Self
        where
            H: ClusterV4ControllerListHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/clusters", on_service(MethodFilter::GET, ClusterV4ControllerListService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/clusters/updates
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerUpdatesPath,super::endpoint::ClusterV4ControllerUpdatesQuery) -> ClusterV4ControllerUpdatesResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_updates(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerUpdatesPath,super::endpoint::ClusterV4ControllerUpdatesQuery, ...extractors) -> ClusterV4ControllerUpdatesResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_updates(handler);
    /// ```
    pub fn cluster_v4_controller_updates<H, T>(mut self, handler: H) -> Self
        where
            H: ClusterV4ControllerUpdatesHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/clusters/updates", on_service(MethodFilter::GET, ClusterV4ControllerUpdatesService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/clusters/{id}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerByIdPath,super::endpoint::ClusterV4ControllerByIdQuery) -> ClusterV4ControllerByIdResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_by_id(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerByIdPath,super::endpoint::ClusterV4ControllerByIdQuery, ...extractors) -> ClusterV4ControllerByIdResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_by_id(handler);
    /// ```
    pub fn cluster_v4_controller_by_id<H, T>(mut self, handler: H) -> Self
        where
            H: ClusterV4ControllerByIdHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/clusters/:id", on_service(MethodFilter::GET, ClusterV4ControllerByIdService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/clusters/owner/{owner}/operators/{operators}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerByOwnerAndOperatorsPath,super::endpoint::ClusterV4ControllerByOwnerAndOperatorsQuery) -> ClusterV4ControllerByOwnerAndOperatorsResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_by_owner_and_operators(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerByOwnerAndOperatorsPath,super::endpoint::ClusterV4ControllerByOwnerAndOperatorsQuery, ...extractors) -> ClusterV4ControllerByOwnerAndOperatorsResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_by_owner_and_operators(handler);
    /// ```
    pub fn cluster_v4_controller_by_owner_and_operators<H, T>(mut self, handler: H) -> Self
        where
            H: ClusterV4ControllerByOwnerAndOperatorsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/clusters/owner/:owner/operators/:operators", on_service(MethodFilter::GET, ClusterV4ControllerByOwnerAndOperatorsService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/clusters/owner/{owner}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerByOwnerPath,super::endpoint::ClusterV4ControllerByOwnerQuery) -> ClusterV4ControllerByOwnerResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_by_owner(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerByOwnerPath,super::endpoint::ClusterV4ControllerByOwnerQuery, ...extractors) -> ClusterV4ControllerByOwnerResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_by_owner(handler);
    /// ```
    pub fn cluster_v4_controller_by_owner<H, T>(mut self, handler: H) -> Self
        where
            H: ClusterV4ControllerByOwnerHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/clusters/owner/:owner", on_service(MethodFilter::GET, ClusterV4ControllerByOwnerService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/clusters/hash/{clusterHash}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerValidatorsPath,super::endpoint::ClusterV4ControllerValidatorsQuery) -> ClusterV4ControllerValidatorsResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_validators(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ClusterV4ControllerValidatorsPath,super::endpoint::ClusterV4ControllerValidatorsQuery, ...extractors) -> ClusterV4ControllerValidatorsResponse {
    ///     todo!();
    /// }
    /// let router = ClustersRouter::default().cluster_v4_controller_validators(handler);
    /// ```
    pub fn cluster_v4_controller_validators<H, T>(mut self, handler: H) -> Self
        where
            H: ClusterV4ControllerValidatorsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/clusters/hash/:clusterHash", on_service(MethodFilter::GET, ClusterV4ControllerValidatorsService::new(handler)));
        self
    }
}

impl<S> Default for ClustersRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<ClustersRouter<S>> for Router<S> {
    fn from(r: ClustersRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/clusters/count handler
pub trait ClusterV4ControllerCountHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ClusterV4ControllerCountHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ClusterV4ControllerCountPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ClusterV4ControllerCountResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerCountPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ClusterV4ControllerCountResponse = res.into();

            response.into()
        })
    }
}

macro_rules! cluster_v4_controller_count_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ClusterV4ControllerCountHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ClusterV4ControllerCountPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ClusterV4ControllerCountResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerCountPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ClusterV4ControllerCountResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(cluster_v4_controller_count_handler);

/// GET /api/v4/{network}/clusters/count service
struct ClusterV4ControllerCountService<H, T, S>
where
    H: ClusterV4ControllerCountHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ClusterV4ControllerCountService<H, T, S>
where
    H: ClusterV4ControllerCountHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ClusterV4ControllerCountService<H, T, S>
where
    H: ClusterV4ControllerCountHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ClusterV4ControllerCountService<H, T, S>
where
    H: ClusterV4ControllerCountHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ClusterV4ControllerCountHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/clusters handler
pub trait ClusterV4ControllerListHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ClusterV4ControllerListHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ClusterV4ControllerListPath,super::endpoint::ClusterV4ControllerListQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ClusterV4ControllerListResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerListPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerListQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ClusterV4ControllerListResponse = res.into();

            response.into()
        })
    }
}

macro_rules! cluster_v4_controller_list_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ClusterV4ControllerListHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ClusterV4ControllerListPath,super::endpoint::ClusterV4ControllerListQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ClusterV4ControllerListResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerListPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerListQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ClusterV4ControllerListResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(cluster_v4_controller_list_handler);

/// GET /api/v4/{network}/clusters service
struct ClusterV4ControllerListService<H, T, S>
where
    H: ClusterV4ControllerListHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ClusterV4ControllerListService<H, T, S>
where
    H: ClusterV4ControllerListHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ClusterV4ControllerListService<H, T, S>
where
    H: ClusterV4ControllerListHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ClusterV4ControllerListService<H, T, S>
where
    H: ClusterV4ControllerListHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ClusterV4ControllerListHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/clusters/updates handler
pub trait ClusterV4ControllerUpdatesHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ClusterV4ControllerUpdatesHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ClusterV4ControllerUpdatesPath,super::endpoint::ClusterV4ControllerUpdatesQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ClusterV4ControllerUpdatesResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerUpdatesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerUpdatesQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ClusterV4ControllerUpdatesResponse = res.into();

            response.into()
        })
    }
}

macro_rules! cluster_v4_controller_updates_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ClusterV4ControllerUpdatesHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ClusterV4ControllerUpdatesPath,super::endpoint::ClusterV4ControllerUpdatesQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ClusterV4ControllerUpdatesResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerUpdatesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerUpdatesQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ClusterV4ControllerUpdatesResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(cluster_v4_controller_updates_handler);

/// GET /api/v4/{network}/clusters/updates service
struct ClusterV4ControllerUpdatesService<H, T, S>
where
    H: ClusterV4ControllerUpdatesHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ClusterV4ControllerUpdatesService<H, T, S>
where
    H: ClusterV4ControllerUpdatesHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ClusterV4ControllerUpdatesService<H, T, S>
where
    H: ClusterV4ControllerUpdatesHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ClusterV4ControllerUpdatesService<H, T, S>
where
    H: ClusterV4ControllerUpdatesHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ClusterV4ControllerUpdatesHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/clusters/{id} handler
pub trait ClusterV4ControllerByIdHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ClusterV4ControllerByIdHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ClusterV4ControllerByIdPath,super::endpoint::ClusterV4ControllerByIdQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ClusterV4ControllerByIdResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerByIdPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerByIdQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ClusterV4ControllerByIdResponse = res.into();

            response.into()
        })
    }
}

macro_rules! cluster_v4_controller_by_id_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ClusterV4ControllerByIdHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ClusterV4ControllerByIdPath,super::endpoint::ClusterV4ControllerByIdQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ClusterV4ControllerByIdResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerByIdPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerByIdQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ClusterV4ControllerByIdResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(cluster_v4_controller_by_id_handler);

/// GET /api/v4/{network}/clusters/{id} service
struct ClusterV4ControllerByIdService<H, T, S>
where
    H: ClusterV4ControllerByIdHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ClusterV4ControllerByIdService<H, T, S>
where
    H: ClusterV4ControllerByIdHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ClusterV4ControllerByIdService<H, T, S>
where
    H: ClusterV4ControllerByIdHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ClusterV4ControllerByIdService<H, T, S>
where
    H: ClusterV4ControllerByIdHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ClusterV4ControllerByIdHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/clusters/owner/{owner}/operators/{operators} handler
pub trait ClusterV4ControllerByOwnerAndOperatorsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ClusterV4ControllerByOwnerAndOperatorsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ClusterV4ControllerByOwnerAndOperatorsPath,super::endpoint::ClusterV4ControllerByOwnerAndOperatorsQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ClusterV4ControllerByOwnerAndOperatorsResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerByOwnerAndOperatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerByOwnerAndOperatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ClusterV4ControllerByOwnerAndOperatorsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! cluster_v4_controller_by_owner_and_operators_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ClusterV4ControllerByOwnerAndOperatorsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ClusterV4ControllerByOwnerAndOperatorsPath,super::endpoint::ClusterV4ControllerByOwnerAndOperatorsQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ClusterV4ControllerByOwnerAndOperatorsResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerByOwnerAndOperatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerByOwnerAndOperatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ClusterV4ControllerByOwnerAndOperatorsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(cluster_v4_controller_by_owner_and_operators_handler);

/// GET /api/v4/{network}/clusters/owner/{owner}/operators/{operators} service
struct ClusterV4ControllerByOwnerAndOperatorsService<H, T, S>
where
    H: ClusterV4ControllerByOwnerAndOperatorsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ClusterV4ControllerByOwnerAndOperatorsService<H, T, S>
where
    H: ClusterV4ControllerByOwnerAndOperatorsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ClusterV4ControllerByOwnerAndOperatorsService<H, T, S>
where
    H: ClusterV4ControllerByOwnerAndOperatorsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ClusterV4ControllerByOwnerAndOperatorsService<H, T, S>
where
    H: ClusterV4ControllerByOwnerAndOperatorsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ClusterV4ControllerByOwnerAndOperatorsHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/clusters/owner/{owner} handler
pub trait ClusterV4ControllerByOwnerHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ClusterV4ControllerByOwnerHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ClusterV4ControllerByOwnerPath,super::endpoint::ClusterV4ControllerByOwnerQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ClusterV4ControllerByOwnerResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerByOwnerPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerByOwnerQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ClusterV4ControllerByOwnerResponse = res.into();

            response.into()
        })
    }
}

macro_rules! cluster_v4_controller_by_owner_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ClusterV4ControllerByOwnerHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ClusterV4ControllerByOwnerPath,super::endpoint::ClusterV4ControllerByOwnerQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ClusterV4ControllerByOwnerResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerByOwnerPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerByOwnerQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ClusterV4ControllerByOwnerResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(cluster_v4_controller_by_owner_handler);

/// GET /api/v4/{network}/clusters/owner/{owner} service
struct ClusterV4ControllerByOwnerService<H, T, S>
where
    H: ClusterV4ControllerByOwnerHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ClusterV4ControllerByOwnerService<H, T, S>
where
    H: ClusterV4ControllerByOwnerHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ClusterV4ControllerByOwnerService<H, T, S>
where
    H: ClusterV4ControllerByOwnerHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ClusterV4ControllerByOwnerService<H, T, S>
where
    H: ClusterV4ControllerByOwnerHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ClusterV4ControllerByOwnerHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/clusters/hash/{clusterHash} handler
pub trait ClusterV4ControllerValidatorsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ClusterV4ControllerValidatorsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ClusterV4ControllerValidatorsPath,super::endpoint::ClusterV4ControllerValidatorsQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ClusterV4ControllerValidatorsResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerValidatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerValidatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ClusterV4ControllerValidatorsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! cluster_v4_controller_validators_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ClusterV4ControllerValidatorsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ClusterV4ControllerValidatorsPath,super::endpoint::ClusterV4ControllerValidatorsQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ClusterV4ControllerValidatorsResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ClusterV4ControllerValidatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ClusterV4ControllerValidatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ClusterV4ControllerValidatorsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(cluster_v4_controller_validators_handler);

/// GET /api/v4/{network}/clusters/hash/{clusterHash} service
struct ClusterV4ControllerValidatorsService<H, T, S>
where
    H: ClusterV4ControllerValidatorsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ClusterV4ControllerValidatorsService<H, T, S>
where
    H: ClusterV4ControllerValidatorsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ClusterV4ControllerValidatorsService<H, T, S>
where
    H: ClusterV4ControllerValidatorsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ClusterV4ControllerValidatorsService<H, T, S>
where
    H: ClusterV4ControllerValidatorsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ClusterV4ControllerValidatorsHandler::call(handler, req, state).await)
        })
    }
}



/// Duties router
pub struct DutiesRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> DutiesRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/duties/{validator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery) -> DutiesV4ControllerDutiesResponse {
    ///     todo!();
    /// }
    /// let router = DutiesRouter::default().duties_v4_controller_duties(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery, ...extractors) -> DutiesV4ControllerDutiesResponse {
    ///     todo!();
    /// }
    /// let router = DutiesRouter::default().duties_v4_controller_duties(handler);
    /// ```
    pub fn duties_v4_controller_duties<H, T>(mut self, handler: H) -> Self
        where
            H: DutiesV4ControllerDutiesHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/duties/:validator", on_service(MethodFilter::GET, DutiesV4ControllerDutiesService::new(handler)));
        self
    }
}

impl<S> Default for DutiesRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<DutiesRouter<S>> for Router<S> {
    fn from(r: DutiesRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/duties/{validator} handler
pub trait DutiesV4ControllerDutiesHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> DutiesV4ControllerDutiesHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::DutiesV4ControllerDutiesResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::DutiesV4ControllerDutiesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::DutiesV4ControllerDutiesQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::DutiesV4ControllerDutiesResponse = res.into();

            response.into()
        })
    }
}

macro_rules! duties_v4_controller_duties_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> DutiesV4ControllerDutiesHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::DutiesV4ControllerDutiesResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::DutiesV4ControllerDutiesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::DutiesV4ControllerDutiesQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::DutiesV4ControllerDutiesResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(duties_v4_controller_duties_handler);

/// GET /api/v4/{network}/duties/{validator} service
struct DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( DutiesV4ControllerDutiesHandler::call(handler, req, state).await)
        })
    }
}



/// Events router
pub struct EventsRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> EventsRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/events/{txHash}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4EventsGetPath) -> ApiV4EventsGetResponse {
    ///     todo!();
    /// }
    /// let router = EventsRouter::default().api_v4_events_get(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4EventsGetPath, ...extractors) -> ApiV4EventsGetResponse {
    ///     todo!();
    /// }
    /// let router = EventsRouter::default().api_v4_events_get(handler);
    /// ```
    pub fn api_v4_events_get<H, T>(mut self, handler: H) -> Self
        where
            H: ApiV4EventsGetHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/events/:txHash", on_service(MethodFilter::GET, ApiV4EventsGetService::new(handler)));
        self
    }
}

impl<S> Default for EventsRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<EventsRouter<S>> for Router<S> {
    fn from(r: EventsRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/events/{txHash} handler
pub trait ApiV4EventsGetHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ApiV4EventsGetHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ApiV4EventsGetPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ApiV4EventsGetResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ApiV4EventsGetPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ApiV4EventsGetResponse = res.into();

            response.into()
        })
    }
}

macro_rules! api_v4_events_get_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ApiV4EventsGetHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ApiV4EventsGetPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ApiV4EventsGetResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ApiV4EventsGetPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ApiV4EventsGetResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(api_v4_events_get_handler);

/// GET /api/v4/{network}/events/{txHash} service
struct ApiV4EventsGetService<H, T, S>
where
    H: ApiV4EventsGetHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ApiV4EventsGetService<H, T, S>
where
    H: ApiV4EventsGetHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ApiV4EventsGetService<H, T, S>
where
    H: ApiV4EventsGetHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ApiV4EventsGetService<H, T, S>
where
    H: ApiV4EventsGetHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ApiV4EventsGetHandler::call(handler, req, state).await)
        })
    }
}



/// Faucet router
pub struct FaucetRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> FaucetRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/faucet
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::FaucetControllerGetTransactionsPath) -> FaucetControllerGetTransactionsResponse {
    ///     todo!();
    /// }
    /// let router = FaucetRouter::default().faucet_controller_get_transactions(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::FaucetControllerGetTransactionsPath, ...extractors) -> FaucetControllerGetTransactionsResponse {
    ///     todo!();
    /// }
    /// let router = FaucetRouter::default().faucet_controller_get_transactions(handler);
    /// ```
    pub fn faucet_controller_get_transactions<H, T>(mut self, handler: H) -> Self
        where
            H: FaucetControllerGetTransactionsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/faucet", on_service(MethodFilter::GET, FaucetControllerGetTransactionsService::new(handler)));
        self
    }

    /// POST /api/v4/{network}/faucet
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::FaucetControllerSetTransactionPath,super::model::FaucetControllerSetTransactionRequestBody) -> FaucetControllerSetTransactionResponse {
    ///     todo!();
    /// }
    /// let router = FaucetRouter::default().faucet_controller_set_transaction(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::FaucetControllerSetTransactionPath,super::model::FaucetControllerSetTransactionRequestBody, ...extractors) -> FaucetControllerSetTransactionResponse {
    ///     todo!();
    /// }
    /// let router = FaucetRouter::default().faucet_controller_set_transaction(handler);
    /// ```
    pub fn faucet_controller_set_transaction<H, T>(mut self, handler: H) -> Self
        where
            H: FaucetControllerSetTransactionHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/faucet", on_service(MethodFilter::POST, FaucetControllerSetTransactionService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/faucet/config
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::FaucetControllerGetFaucetConfigPath) -> FaucetControllerGetFaucetConfigResponse {
    ///     todo!();
    /// }
    /// let router = FaucetRouter::default().faucet_controller_get_faucet_config(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::FaucetControllerGetFaucetConfigPath, ...extractors) -> FaucetControllerGetFaucetConfigResponse {
    ///     todo!();
    /// }
    /// let router = FaucetRouter::default().faucet_controller_get_faucet_config(handler);
    /// ```
    pub fn faucet_controller_get_faucet_config<H, T>(mut self, handler: H) -> Self
        where
            H: FaucetControllerGetFaucetConfigHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/faucet/config", on_service(MethodFilter::GET, FaucetControllerGetFaucetConfigService::new(handler)));
        self
    }
}

impl<S> Default for FaucetRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<FaucetRouter<S>> for Router<S> {
    fn from(r: FaucetRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/faucet handler
pub trait FaucetControllerGetTransactionsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> FaucetControllerGetTransactionsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::FaucetControllerGetTransactionsPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::FaucetControllerGetTransactionsResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::FaucetControllerGetTransactionsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::FaucetControllerGetTransactionsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! faucet_controller_get_transactions_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> FaucetControllerGetTransactionsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::FaucetControllerGetTransactionsPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::FaucetControllerGetTransactionsResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::FaucetControllerGetTransactionsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::FaucetControllerGetTransactionsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(faucet_controller_get_transactions_handler);

/// GET /api/v4/{network}/faucet service
struct FaucetControllerGetTransactionsService<H, T, S>
where
    H: FaucetControllerGetTransactionsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for FaucetControllerGetTransactionsService<H, T, S>
where
    H: FaucetControllerGetTransactionsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> FaucetControllerGetTransactionsService<H, T, S>
where
    H: FaucetControllerGetTransactionsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for FaucetControllerGetTransactionsService<H, T, S>
where
    H: FaucetControllerGetTransactionsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( FaucetControllerGetTransactionsHandler::call(handler, req, state).await)
        })
    }
}


/// POST /api/v4/{network}/faucet handler
pub trait FaucetControllerSetTransactionHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> FaucetControllerSetTransactionHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::FaucetControllerSetTransactionPath,super::model::FaucetControllerSetTransactionRequestBody,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::FaucetControllerSetTransactionResponse>,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::FaucetControllerSetTransactionPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::FaucetControllerSetTransactionRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

            let res = self(path.0, body.0, ).await;

            let response: super::endpoint::FaucetControllerSetTransactionResponse = res.into();

            response.into()
        })
    }
}

macro_rules! faucet_controller_set_transaction_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> FaucetControllerSetTransactionHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::FaucetControllerSetTransactionPath,super::model::FaucetControllerSetTransactionRequestBody,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::FaucetControllerSetTransactionResponse>,B: HttpBody + Send + 'static,
            B::Data: Send,
            B::Error: Into<BoxError>,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::FaucetControllerSetTransactionPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    
                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::FaucetControllerSetTransactionRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let res = self(path.0, body.0, $($ty,)*).await;

                    let response: super::endpoint::FaucetControllerSetTransactionResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(faucet_controller_set_transaction_handler);

/// POST /api/v4/{network}/faucet service
struct FaucetControllerSetTransactionService<H, T, S>
where
    H: FaucetControllerSetTransactionHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for FaucetControllerSetTransactionService<H, T, S>
where
    H: FaucetControllerSetTransactionHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> FaucetControllerSetTransactionService<H, T, S>
where
    H: FaucetControllerSetTransactionHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for FaucetControllerSetTransactionService<H, T, S>
where
    H: FaucetControllerSetTransactionHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( FaucetControllerSetTransactionHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/faucet/config handler
pub trait FaucetControllerGetFaucetConfigHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> FaucetControllerGetFaucetConfigHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::FaucetControllerGetFaucetConfigPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::FaucetControllerGetFaucetConfigResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::FaucetControllerGetFaucetConfigPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::FaucetControllerGetFaucetConfigResponse = res.into();

            response.into()
        })
    }
}

macro_rules! faucet_controller_get_faucet_config_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> FaucetControllerGetFaucetConfigHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::FaucetControllerGetFaucetConfigPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::FaucetControllerGetFaucetConfigResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::FaucetControllerGetFaucetConfigPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::FaucetControllerGetFaucetConfigResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(faucet_controller_get_faucet_config_handler);

/// GET /api/v4/{network}/faucet/config service
struct FaucetControllerGetFaucetConfigService<H, T, S>
where
    H: FaucetControllerGetFaucetConfigHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for FaucetControllerGetFaucetConfigService<H, T, S>
where
    H: FaucetControllerGetFaucetConfigHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> FaucetControllerGetFaucetConfigService<H, T, S>
where
    H: FaucetControllerGetFaucetConfigHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for FaucetControllerGetFaucetConfigService<H, T, S>
where
    H: FaucetControllerGetFaucetConfigHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( FaucetControllerGetFaucetConfigHandler::call(handler, req, state).await)
        })
    }
}



/// Finance router
pub struct FinanceRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> FinanceRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/finance/currency/convert/{symbol}/{quote}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::FinanceControllerCurrencyConvertPath) -> FinanceControllerCurrencyConvertResponse {
    ///     todo!();
    /// }
    /// let router = FinanceRouter::default().finance_controller_currency_convert(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::FinanceControllerCurrencyConvertPath, ...extractors) -> FinanceControllerCurrencyConvertResponse {
    ///     todo!();
    /// }
    /// let router = FinanceRouter::default().finance_controller_currency_convert(handler);
    /// ```
    pub fn finance_controller_currency_convert<H, T>(mut self, handler: H) -> Self
        where
            H: FinanceControllerCurrencyConvertHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/finance/currency/convert/:symbol/:quote", on_service(MethodFilter::GET, FinanceControllerCurrencyConvertService::new(handler)));
        self
    }
}

impl<S> Default for FinanceRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<FinanceRouter<S>> for Router<S> {
    fn from(r: FinanceRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/finance/currency/convert/{symbol}/{quote} handler
pub trait FinanceControllerCurrencyConvertHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> FinanceControllerCurrencyConvertHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::FinanceControllerCurrencyConvertPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::FinanceControllerCurrencyConvertResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::FinanceControllerCurrencyConvertPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::FinanceControllerCurrencyConvertResponse = res.into();

            response.into()
        })
    }
}

macro_rules! finance_controller_currency_convert_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> FinanceControllerCurrencyConvertHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::FinanceControllerCurrencyConvertPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::FinanceControllerCurrencyConvertResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::FinanceControllerCurrencyConvertPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::FinanceControllerCurrencyConvertResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(finance_controller_currency_convert_handler);

/// GET /api/finance/currency/convert/{symbol}/{quote} service
struct FinanceControllerCurrencyConvertService<H, T, S>
where
    H: FinanceControllerCurrencyConvertHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for FinanceControllerCurrencyConvertService<H, T, S>
where
    H: FinanceControllerCurrencyConvertHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> FinanceControllerCurrencyConvertService<H, T, S>
where
    H: FinanceControllerCurrencyConvertHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for FinanceControllerCurrencyConvertService<H, T, S>
where
    H: FinanceControllerCurrencyConvertHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( FinanceControllerCurrencyConvertHandler::call(handler, req, state).await)
        })
    }
}



/// Graph router
pub struct GraphRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> GraphRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/operators/graph
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery) -> OperatorsV4ControllerGraphResponse {
    ///     todo!();
    /// }
    /// let router = GraphRouter::default().operators_v4_controller_graph(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery, ...extractors) -> OperatorsV4ControllerGraphResponse {
    ///     todo!();
    /// }
    /// let router = GraphRouter::default().operators_v4_controller_graph(handler);
    /// ```
    pub fn operators_v4_controller_graph<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerGraphHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/graph", on_service(MethodFilter::GET, OperatorsV4ControllerGraphService::new(handler)));
        self
    }
}

impl<S> Default for GraphRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<GraphRouter<S>> for Router<S> {
    fn from(r: GraphRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/operators/graph handler
pub trait OperatorsV4ControllerGraphHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerGraphHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerGraphResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGraphPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerGraphQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerGraphResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_graph_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerGraphHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerGraphResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGraphPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerGraphQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerGraphResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_graph_handler);

/// GET /api/v4/{network}/operators/graph service
struct OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerGraphHandler::call(handler, req, state).await)
        })
    }
}



/// Health router
pub struct HealthRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> HealthRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/health
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::HealthV4ControllerHealthPath) -> HealthV4ControllerHealthResponse {
    ///     todo!();
    /// }
    /// let router = HealthRouter::default().health_v4_controller_health(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::HealthV4ControllerHealthPath, ...extractors) -> HealthV4ControllerHealthResponse {
    ///     todo!();
    /// }
    /// let router = HealthRouter::default().health_v4_controller_health(handler);
    /// ```
    pub fn health_v4_controller_health<H, T>(mut self, handler: H) -> Self
        where
            H: HealthV4ControllerHealthHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/health", on_service(MethodFilter::GET, HealthV4ControllerHealthService::new(handler)));
        self
    }
}

impl<S> Default for HealthRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<HealthRouter<S>> for Router<S> {
    fn from(r: HealthRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/health handler
pub trait HealthV4ControllerHealthHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> HealthV4ControllerHealthHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::HealthV4ControllerHealthPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::HealthV4ControllerHealthResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::HealthV4ControllerHealthPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::HealthV4ControllerHealthResponse = res.into();

            response.into()
        })
    }
}

macro_rules! health_v4_controller_health_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> HealthV4ControllerHealthHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::HealthV4ControllerHealthPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::HealthV4ControllerHealthResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::HealthV4ControllerHealthPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::HealthV4ControllerHealthResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(health_v4_controller_health_handler);

/// GET /api/v4/{network}/health service
struct HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( HealthV4ControllerHealthHandler::call(handler, req, state).await)
        })
    }
}



/// Incentivized router
pub struct IncentivizedRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> IncentivizedRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/incentivization/merkle-tree
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath) -> IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
    ///     todo!();
    /// }
    /// let router = IncentivizedRouter::default().incentivization_v4_controller_migration_operators_distribution(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath, ...extractors) -> IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
    ///     todo!();
    /// }
    /// let router = IncentivizedRouter::default().incentivization_v4_controller_migration_operators_distribution(handler);
    /// ```
    pub fn incentivization_v4_controller_migration_operators_distribution<H, T>(mut self, handler: H) -> Self
        where
            H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/incentivization/merkle-tree", on_service(MethodFilter::GET, IncentivizationV4ControllerMigrationOperatorsDistributionService::new(handler)));
        self
    }
}

impl<S> Default for IncentivizedRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<IncentivizedRouter<S>> for Router<S> {
    fn from(r: IncentivizedRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/incentivization/merkle-tree handler
pub trait IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> IncentivizationV4ControllerMigrationOperatorsDistributionHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse = res.into();

            response.into()
        })
    }
}

macro_rules! incentivization_v4_controller_migration_operators_distribution_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> IncentivizationV4ControllerMigrationOperatorsDistributionHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(incentivization_v4_controller_migration_operators_distribution_handler);

/// GET /api/v4/{network}/incentivization/merkle-tree service
struct IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( IncentivizationV4ControllerMigrationOperatorsDistributionHandler::call(handler, req, state).await)
        })
    }
}



/// Operators router
pub struct OperatorsRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> OperatorsRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/operators/graph
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery) -> OperatorsV4ControllerGraphResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_graph(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery, ...extractors) -> OperatorsV4ControllerGraphResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_graph(handler);
    /// ```
    pub fn operators_v4_controller_graph<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerGraphHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/graph", on_service(MethodFilter::GET, OperatorsV4ControllerGraphService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/operators/owned_by/{ownerAddress}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerOwnedByPath,super::endpoint::OperatorsV4ControllerOwnedByQuery) -> OperatorsV4ControllerOwnedByResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_owned_by(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerOwnedByPath,super::endpoint::OperatorsV4ControllerOwnedByQuery, ...extractors) -> OperatorsV4ControllerOwnedByResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_owned_by(handler);
    /// ```
    pub fn operators_v4_controller_owned_by<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerOwnedByHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/owned_by/:ownerAddress", on_service(MethodFilter::GET, OperatorsV4ControllerOwnedByService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/operators/incentivized/{operator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerIncentivizedPath,super::endpoint::OperatorsV4ControllerIncentivizedQuery) -> OperatorsV4ControllerIncentivizedResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_incentivized(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerIncentivizedPath,super::endpoint::OperatorsV4ControllerIncentivizedQuery, ...extractors) -> OperatorsV4ControllerIncentivizedResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_incentivized(handler);
    /// ```
    pub fn operators_v4_controller_incentivized<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerIncentivizedHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/incentivized/:operator", on_service(MethodFilter::GET, OperatorsV4ControllerIncentivizedService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/operators/{operator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetOperatorPath) -> OperatorsV4ControllerGetOperatorResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_operator(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetOperatorPath, ...extractors) -> OperatorsV4ControllerGetOperatorResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_operator(handler);
    /// ```
    pub fn operators_v4_controller_get_operator<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerGetOperatorHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/:operator", on_service(MethodFilter::GET, OperatorsV4ControllerGetOperatorService::new(handler)));
        self
    }

    /// POST /api/v4/{network}/operators/dkg_health_check
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetDkgHealthCheckPath,super::model::OperatorsV4ControllerGetDkgHealthCheckRequestBody) -> OperatorsV4ControllerGetDkgHealthCheckResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_dkg_health_check(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetDkgHealthCheckPath,super::model::OperatorsV4ControllerGetDkgHealthCheckRequestBody, ...extractors) -> OperatorsV4ControllerGetDkgHealthCheckResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_dkg_health_check(handler);
    /// ```
    pub fn operators_v4_controller_get_dkg_health_check<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerGetDkgHealthCheckHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/dkg_health_check", on_service(MethodFilter::POST, OperatorsV4ControllerGetDkgHealthCheckService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/operators/public_key/{public_key}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetByPublicKeyPath) -> OperatorsV4ControllerGetByPublicKeyResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_by_public_key(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetByPublicKeyPath, ...extractors) -> OperatorsV4ControllerGetByPublicKeyResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_by_public_key(handler);
    /// ```
    pub fn operators_v4_controller_get_by_public_key<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerGetByPublicKeyHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/public_key/:public_key", on_service(MethodFilter::GET, OperatorsV4ControllerGetByPublicKeyService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/operators
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerOperatorsPath,super::endpoint::OperatorsV4ControllerOperatorsQuery) -> OperatorsV4ControllerOperatorsResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_operators(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerOperatorsPath,super::endpoint::OperatorsV4ControllerOperatorsQuery, ...extractors) -> OperatorsV4ControllerOperatorsResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_operators(handler);
    /// ```
    pub fn operators_v4_controller_operators<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerOperatorsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators", on_service(MethodFilter::GET, OperatorsV4ControllerOperatorsService::new(handler)));
        self
    }

    /// POST /api/v4/{network}/operators
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetByIdsPath,super::model::OperatorsV4ControllerGetByIdsRequestBody) -> OperatorsV4ControllerGetByIdsResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_by_ids(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerGetByIdsPath,super::model::OperatorsV4ControllerGetByIdsRequestBody, ...extractors) -> OperatorsV4ControllerGetByIdsResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_get_by_ids(handler);
    /// ```
    pub fn operators_v4_controller_get_by_ids<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerGetByIdsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators", on_service(MethodFilter::POST, OperatorsV4ControllerGetByIdsService::new(handler)));
        self
    }

    /// PUT /api/v4/{network}/operators/{operator}/metadata
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerUpdateMetadataPath,super::model::OperatorsV4ControllerUpdateMetadataRequestBody) -> OperatorsV4ControllerUpdateMetadataResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_update_metadata(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerUpdateMetadataPath,super::model::OperatorsV4ControllerUpdateMetadataRequestBody, ...extractors) -> OperatorsV4ControllerUpdateMetadataResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_update_metadata(handler);
    /// ```
    pub fn operators_v4_controller_update_metadata<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerUpdateMetadataHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/:operator/metadata", on_service(MethodFilter::PUT, OperatorsV4ControllerUpdateMetadataService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/operators/nodes/{layer}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerNodesPath) -> OperatorsV4ControllerNodesResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_nodes(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerNodesPath, ...extractors) -> OperatorsV4ControllerNodesResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_nodes(handler);
    /// ```
    pub fn operators_v4_controller_nodes<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerNodesHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/nodes/:layer", on_service(MethodFilter::GET, OperatorsV4ControllerNodesService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/operators/locations
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerLocationsPath) -> OperatorsV4ControllerLocationsResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_locations(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::OperatorsV4ControllerLocationsPath, ...extractors) -> OperatorsV4ControllerLocationsResponse {
    ///     todo!();
    /// }
    /// let router = OperatorsRouter::default().operators_v4_controller_locations(handler);
    /// ```
    pub fn operators_v4_controller_locations<H, T>(mut self, handler: H) -> Self
        where
            H: OperatorsV4ControllerLocationsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/operators/locations", on_service(MethodFilter::GET, OperatorsV4ControllerLocationsService::new(handler)));
        self
    }
}

impl<S> Default for OperatorsRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<OperatorsRouter<S>> for Router<S> {
    fn from(r: OperatorsRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/operators/graph handler
pub trait OperatorsV4ControllerGraphHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerGraphHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerGraphResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGraphPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerGraphQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerGraphResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_graph_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerGraphHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerGraphPath,super::endpoint::OperatorsV4ControllerGraphQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerGraphResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGraphPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerGraphQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerGraphResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_graph_handler);

/// GET /api/v4/{network}/operators/graph service
struct OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerGraphService<H, T, S>
where
    H: OperatorsV4ControllerGraphHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerGraphHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/operators/owned_by/{ownerAddress} handler
pub trait OperatorsV4ControllerOwnedByHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerOwnedByHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerOwnedByPath,super::endpoint::OperatorsV4ControllerOwnedByQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerOwnedByResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerOwnedByPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerOwnedByQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerOwnedByResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_owned_by_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerOwnedByHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerOwnedByPath,super::endpoint::OperatorsV4ControllerOwnedByQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerOwnedByResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerOwnedByPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerOwnedByQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerOwnedByResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_owned_by_handler);

/// GET /api/v4/{network}/operators/owned_by/{ownerAddress} service
struct OperatorsV4ControllerOwnedByService<H, T, S>
where
    H: OperatorsV4ControllerOwnedByHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerOwnedByService<H, T, S>
where
    H: OperatorsV4ControllerOwnedByHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerOwnedByService<H, T, S>
where
    H: OperatorsV4ControllerOwnedByHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerOwnedByService<H, T, S>
where
    H: OperatorsV4ControllerOwnedByHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerOwnedByHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/operators/incentivized/{operator} handler
pub trait OperatorsV4ControllerIncentivizedHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerIncentivizedHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerIncentivizedPath,super::endpoint::OperatorsV4ControllerIncentivizedQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerIncentivizedResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerIncentivizedPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerIncentivizedQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerIncentivizedResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_incentivized_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerIncentivizedHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerIncentivizedPath,super::endpoint::OperatorsV4ControllerIncentivizedQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerIncentivizedResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerIncentivizedPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerIncentivizedQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerIncentivizedResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_incentivized_handler);

/// GET /api/v4/{network}/operators/incentivized/{operator} service
struct OperatorsV4ControllerIncentivizedService<H, T, S>
where
    H: OperatorsV4ControllerIncentivizedHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerIncentivizedService<H, T, S>
where
    H: OperatorsV4ControllerIncentivizedHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerIncentivizedService<H, T, S>
where
    H: OperatorsV4ControllerIncentivizedHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerIncentivizedService<H, T, S>
where
    H: OperatorsV4ControllerIncentivizedHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerIncentivizedHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/operators/{operator} handler
pub trait OperatorsV4ControllerGetOperatorHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerGetOperatorHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerGetOperatorPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetOperatorResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetOperatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerGetOperatorResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_get_operator_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerGetOperatorHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerGetOperatorPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetOperatorResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetOperatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerGetOperatorResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_get_operator_handler);

/// GET /api/v4/{network}/operators/{operator} service
struct OperatorsV4ControllerGetOperatorService<H, T, S>
where
    H: OperatorsV4ControllerGetOperatorHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerGetOperatorService<H, T, S>
where
    H: OperatorsV4ControllerGetOperatorHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerGetOperatorService<H, T, S>
where
    H: OperatorsV4ControllerGetOperatorHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerGetOperatorService<H, T, S>
where
    H: OperatorsV4ControllerGetOperatorHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerGetOperatorHandler::call(handler, req, state).await)
        })
    }
}


/// POST /api/v4/{network}/operators/dkg_health_check handler
pub trait OperatorsV4ControllerGetDkgHealthCheckHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerGetDkgHealthCheckHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerGetDkgHealthCheckPath,super::model::OperatorsV4ControllerGetDkgHealthCheckRequestBody,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetDkgHealthCheckResponse>,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetDkgHealthCheckPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::OperatorsV4ControllerGetDkgHealthCheckRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

            let res = self(path.0, body.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerGetDkgHealthCheckResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_get_dkg_health_check_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerGetDkgHealthCheckHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerGetDkgHealthCheckPath,super::model::OperatorsV4ControllerGetDkgHealthCheckRequestBody,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetDkgHealthCheckResponse>,B: HttpBody + Send + 'static,
            B::Data: Send,
            B::Error: Into<BoxError>,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetDkgHealthCheckPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    
                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::OperatorsV4ControllerGetDkgHealthCheckRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let res = self(path.0, body.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerGetDkgHealthCheckResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_get_dkg_health_check_handler);

/// POST /api/v4/{network}/operators/dkg_health_check service
struct OperatorsV4ControllerGetDkgHealthCheckService<H, T, S>
where
    H: OperatorsV4ControllerGetDkgHealthCheckHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerGetDkgHealthCheckService<H, T, S>
where
    H: OperatorsV4ControllerGetDkgHealthCheckHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerGetDkgHealthCheckService<H, T, S>
where
    H: OperatorsV4ControllerGetDkgHealthCheckHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerGetDkgHealthCheckService<H, T, S>
where
    H: OperatorsV4ControllerGetDkgHealthCheckHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerGetDkgHealthCheckHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/operators/public_key/{public_key} handler
pub trait OperatorsV4ControllerGetByPublicKeyHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerGetByPublicKeyHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerGetByPublicKeyPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetByPublicKeyResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetByPublicKeyPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerGetByPublicKeyResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_get_by_public_key_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerGetByPublicKeyHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerGetByPublicKeyPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetByPublicKeyResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetByPublicKeyPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerGetByPublicKeyResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_get_by_public_key_handler);

/// GET /api/v4/{network}/operators/public_key/{public_key} service
struct OperatorsV4ControllerGetByPublicKeyService<H, T, S>
where
    H: OperatorsV4ControllerGetByPublicKeyHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerGetByPublicKeyService<H, T, S>
where
    H: OperatorsV4ControllerGetByPublicKeyHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerGetByPublicKeyService<H, T, S>
where
    H: OperatorsV4ControllerGetByPublicKeyHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerGetByPublicKeyService<H, T, S>
where
    H: OperatorsV4ControllerGetByPublicKeyHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerGetByPublicKeyHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/operators handler
pub trait OperatorsV4ControllerOperatorsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerOperatorsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerOperatorsPath,super::endpoint::OperatorsV4ControllerOperatorsQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerOperatorsResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerOperatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerOperatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerOperatorsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_operators_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerOperatorsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerOperatorsPath,super::endpoint::OperatorsV4ControllerOperatorsQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerOperatorsResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerOperatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::OperatorsV4ControllerOperatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerOperatorsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_operators_handler);

/// GET /api/v4/{network}/operators service
struct OperatorsV4ControllerOperatorsService<H, T, S>
where
    H: OperatorsV4ControllerOperatorsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerOperatorsService<H, T, S>
where
    H: OperatorsV4ControllerOperatorsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerOperatorsService<H, T, S>
where
    H: OperatorsV4ControllerOperatorsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerOperatorsService<H, T, S>
where
    H: OperatorsV4ControllerOperatorsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerOperatorsHandler::call(handler, req, state).await)
        })
    }
}


/// POST /api/v4/{network}/operators handler
pub trait OperatorsV4ControllerGetByIdsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerGetByIdsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerGetByIdsPath,super::model::OperatorsV4ControllerGetByIdsRequestBody,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetByIdsResponse>,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetByIdsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::OperatorsV4ControllerGetByIdsRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

            let res = self(path.0, body.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerGetByIdsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_get_by_ids_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerGetByIdsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerGetByIdsPath,super::model::OperatorsV4ControllerGetByIdsRequestBody,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerGetByIdsResponse>,B: HttpBody + Send + 'static,
            B::Data: Send,
            B::Error: Into<BoxError>,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerGetByIdsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    
                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::OperatorsV4ControllerGetByIdsRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let res = self(path.0, body.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerGetByIdsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_get_by_ids_handler);

/// POST /api/v4/{network}/operators service
struct OperatorsV4ControllerGetByIdsService<H, T, S>
where
    H: OperatorsV4ControllerGetByIdsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerGetByIdsService<H, T, S>
where
    H: OperatorsV4ControllerGetByIdsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerGetByIdsService<H, T, S>
where
    H: OperatorsV4ControllerGetByIdsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerGetByIdsService<H, T, S>
where
    H: OperatorsV4ControllerGetByIdsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerGetByIdsHandler::call(handler, req, state).await)
        })
    }
}


/// PUT /api/v4/{network}/operators/{operator}/metadata handler
pub trait OperatorsV4ControllerUpdateMetadataHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerUpdateMetadataHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerUpdateMetadataPath,super::model::OperatorsV4ControllerUpdateMetadataRequestBody,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerUpdateMetadataResponse>,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerUpdateMetadataPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::OperatorsV4ControllerUpdateMetadataRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

            let res = self(path.0, body.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerUpdateMetadataResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_update_metadata_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerUpdateMetadataHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerUpdateMetadataPath,super::model::OperatorsV4ControllerUpdateMetadataRequestBody,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerUpdateMetadataResponse>,B: HttpBody + Send + 'static,
            B::Data: Send,
            B::Error: Into<BoxError>,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerUpdateMetadataPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    
                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::OperatorsV4ControllerUpdateMetadataRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let res = self(path.0, body.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerUpdateMetadataResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_update_metadata_handler);

/// PUT /api/v4/{network}/operators/{operator}/metadata service
struct OperatorsV4ControllerUpdateMetadataService<H, T, S>
where
    H: OperatorsV4ControllerUpdateMetadataHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerUpdateMetadataService<H, T, S>
where
    H: OperatorsV4ControllerUpdateMetadataHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerUpdateMetadataService<H, T, S>
where
    H: OperatorsV4ControllerUpdateMetadataHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerUpdateMetadataService<H, T, S>
where
    H: OperatorsV4ControllerUpdateMetadataHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerUpdateMetadataHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/operators/nodes/{layer} handler
pub trait OperatorsV4ControllerNodesHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerNodesHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerNodesPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerNodesResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerNodesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerNodesResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_nodes_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerNodesHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerNodesPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerNodesResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerNodesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerNodesResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_nodes_handler);

/// GET /api/v4/{network}/operators/nodes/{layer} service
struct OperatorsV4ControllerNodesService<H, T, S>
where
    H: OperatorsV4ControllerNodesHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerNodesService<H, T, S>
where
    H: OperatorsV4ControllerNodesHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerNodesService<H, T, S>
where
    H: OperatorsV4ControllerNodesHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerNodesService<H, T, S>
where
    H: OperatorsV4ControllerNodesHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerNodesHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/operators/locations handler
pub trait OperatorsV4ControllerLocationsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> OperatorsV4ControllerLocationsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::OperatorsV4ControllerLocationsPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::OperatorsV4ControllerLocationsResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerLocationsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::OperatorsV4ControllerLocationsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! operators_v4_controller_locations_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> OperatorsV4ControllerLocationsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::OperatorsV4ControllerLocationsPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::OperatorsV4ControllerLocationsResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::OperatorsV4ControllerLocationsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::OperatorsV4ControllerLocationsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(operators_v4_controller_locations_handler);

/// GET /api/v4/{network}/operators/locations service
struct OperatorsV4ControllerLocationsService<H, T, S>
where
    H: OperatorsV4ControllerLocationsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for OperatorsV4ControllerLocationsService<H, T, S>
where
    H: OperatorsV4ControllerLocationsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> OperatorsV4ControllerLocationsService<H, T, S>
where
    H: OperatorsV4ControllerLocationsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for OperatorsV4ControllerLocationsService<H, T, S>
where
    H: OperatorsV4ControllerLocationsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( OperatorsV4ControllerLocationsHandler::call(handler, req, state).await)
        })
    }
}



/// Search router
pub struct SearchRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> SearchRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/search
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery) -> SearchV4ControllerSearchResponse {
    ///     todo!();
    /// }
    /// let router = SearchRouter::default().search_v4_controller_search(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery, ...extractors) -> SearchV4ControllerSearchResponse {
    ///     todo!();
    /// }
    /// let router = SearchRouter::default().search_v4_controller_search(handler);
    /// ```
    pub fn search_v4_controller_search<H, T>(mut self, handler: H) -> Self
        where
            H: SearchV4ControllerSearchHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/search", on_service(MethodFilter::GET, SearchV4ControllerSearchService::new(handler)));
        self
    }
}

impl<S> Default for SearchRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<SearchRouter<S>> for Router<S> {
    fn from(r: SearchRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/search handler
pub trait SearchV4ControllerSearchHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> SearchV4ControllerSearchHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::SearchV4ControllerSearchResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::SearchV4ControllerSearchPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::SearchV4ControllerSearchQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::SearchV4ControllerSearchResponse = res.into();

            response.into()
        })
    }
}

macro_rules! search_v4_controller_search_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> SearchV4ControllerSearchHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::SearchV4ControllerSearchResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::SearchV4ControllerSearchPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::SearchV4ControllerSearchQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::SearchV4ControllerSearchResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(search_v4_controller_search_handler);

/// GET /api/v4/{network}/search service
struct SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( SearchV4ControllerSearchHandler::call(handler, req, state).await)
        })
    }
}



/// Validators router
pub struct ValidatorsRouter<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> ValidatorsRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/validators/countActiveValidators
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::CountActiveValidatorsInNetworkPath) -> CountActiveValidatorsInNetworkResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().count_active_validators_in_network(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::CountActiveValidatorsInNetworkPath, ...extractors) -> CountActiveValidatorsInNetworkResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().count_active_validators_in_network(handler);
    /// ```
    pub fn count_active_validators_in_network<H, T>(mut self, handler: H) -> Self
        where
            H: CountActiveValidatorsInNetworkHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/countActiveValidators", on_service(MethodFilter::GET, CountActiveValidatorsInNetworkService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators/owned_by/{ownerAddress}/cost
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerCostPath) -> ValidatorsV4ControllerCostResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_cost(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerCostPath, ...extractors) -> ValidatorsV4ControllerCostResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_cost(handler);
    /// ```
    pub fn validators_v4_controller_cost<H, T>(mut self, handler: H) -> Self
        where
            H: ValidatorsV4ControllerCostHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/owned_by/:ownerAddress/cost", on_service(MethodFilter::GET, ValidatorsV4ControllerCostService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators/in_operator/{operator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerInOperatorPath,super::endpoint::ValidatorsV4ControllerInOperatorQuery) -> ValidatorsV4ControllerInOperatorResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_in_operator(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerInOperatorPath,super::endpoint::ValidatorsV4ControllerInOperatorQuery, ...extractors) -> ValidatorsV4ControllerInOperatorResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_in_operator(handler);
    /// ```
    pub fn validators_v4_controller_in_operator<H, T>(mut self, handler: H) -> Self
        where
            H: ValidatorsV4ControllerInOperatorHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/in_operator/:operator", on_service(MethodFilter::GET, ValidatorsV4ControllerInOperatorService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators/incentivized/{validator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerIncentivizedPath,super::endpoint::ValidatorsV4ControllerIncentivizedQuery) -> ValidatorsV4ControllerIncentivizedResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_incentivized(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerIncentivizedPath,super::endpoint::ValidatorsV4ControllerIncentivizedQuery, ...extractors) -> ValidatorsV4ControllerIncentivizedResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_incentivized(handler);
    /// ```
    pub fn validators_v4_controller_incentivized<H, T>(mut self, handler: H) -> Self
        where
            H: ValidatorsV4ControllerIncentivizedHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/incentivized/:validator", on_service(MethodFilter::GET, ValidatorsV4ControllerIncentivizedService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators/{validator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerValidatorPath) -> ValidatorsV4ControllerValidatorResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_validator(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerValidatorPath, ...extractors) -> ValidatorsV4ControllerValidatorResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_validator(handler);
    /// ```
    pub fn validators_v4_controller_validator<H, T>(mut self, handler: H) -> Self
        where
            H: ValidatorsV4ControllerValidatorHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/:validator", on_service(MethodFilter::GET, ValidatorsV4ControllerValidatorService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators/isRegisteredValidator/{validator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorPath) -> ValidatorsV4ControllerIsRegisteredValidatorResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_is_registered_validator(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorPath, ...extractors) -> ValidatorsV4ControllerIsRegisteredValidatorResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_is_registered_validator(handler);
    /// ```
    pub fn validators_v4_controller_is_registered_validator<H, T>(mut self, handler: H) -> Self
        where
            H: ValidatorsV4ControllerIsRegisteredValidatorHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/isRegisteredValidator/:validator", on_service(MethodFilter::GET, ValidatorsV4ControllerIsRegisteredValidatorService::new(handler)));
        self
    }

    /// POST /api/v4/{network}/validators/registeredByPublicKeys
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreatePath) -> ApiV4ValidatorRegisteredByPublicKeyCreateResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().api_v4_validator_registered_by_public_key_create(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreatePath, ...extractors) -> ApiV4ValidatorRegisteredByPublicKeyCreateResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().api_v4_validator_registered_by_public_key_create(handler);
    /// ```
    pub fn api_v4_validator_registered_by_public_key_create<H, T>(mut self, handler: H) -> Self
        where
            H: ApiV4ValidatorRegisteredByPublicKeyCreateHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/registeredByPublicKeys", on_service(MethodFilter::POST, ApiV4ValidatorRegisteredByPublicKeyCreateService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerValidatorsPath,super::endpoint::ValidatorsV4ControllerValidatorsQuery) -> ValidatorsV4ControllerValidatorsResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_validators(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerValidatorsPath,super::endpoint::ValidatorsV4ControllerValidatorsQuery, ...extractors) -> ValidatorsV4ControllerValidatorsResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_validators(handler);
    /// ```
    pub fn validators_v4_controller_validators<H, T>(mut self, handler: H) -> Self
        where
            H: ValidatorsV4ControllerValidatorsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators", on_service(MethodFilter::GET, ValidatorsV4ControllerValidatorsService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerDutyCountsPath) -> ValidatorsV4ControllerDutyCountsResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_duty_counts(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ValidatorsV4ControllerDutyCountsPath, ...extractors) -> ValidatorsV4ControllerDutyCountsResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().validators_v4_controller_duty_counts(handler);
    /// ```
    pub fn validators_v4_controller_duty_counts<H, T>(mut self, handler: H) -> Self
        where
            H: ValidatorsV4ControllerDutyCountsHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/duty_counts/:from_epoch/:to_epoch", on_service(MethodFilter::GET, ValidatorsV4ControllerDutyCountsService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/validators/validatorsByClusterHash/{clusterHash}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetPath) -> ApiV4ValidatorsValidatorsByClusterHashGetResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().api_v4_validators_validators_by_cluster_hash_get(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetPath, ...extractors) -> ApiV4ValidatorsValidatorsByClusterHashGetResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().api_v4_validators_validators_by_cluster_hash_get(handler);
    /// ```
    pub fn api_v4_validators_validators_by_cluster_hash_get<H, T>(mut self, handler: H) -> Self
        where
            H: ApiV4ValidatorsValidatorsByClusterHashGetHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/validatorsByClusterHash/:clusterHash", on_service(MethodFilter::GET, ApiV4ValidatorsValidatorsByClusterHashGetService::new(handler)));
        self
    }

    /// POST /api/v4/{network}/validators/validatorsWithdrawCredentials
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreatePath,super::model::ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody) -> ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().api_v4_validator_validators_withdraw_credential_create(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreatePath,super::model::ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody, ...extractors) -> ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse {
    ///     todo!();
    /// }
    /// let router = ValidatorsRouter::default().api_v4_validator_validators_withdraw_credential_create(handler);
    /// ```
    pub fn api_v4_validator_validators_withdraw_credential_create<H, T>(mut self, handler: H) -> Self
        where
            H: ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/validators/validatorsWithdrawCredentials", on_service(MethodFilter::POST, ApiV4ValidatorValidatorsWithdrawCredentialCreateService::new(handler)));
        self
    }
}

impl<S> Default for ValidatorsRouter<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<ValidatorsRouter<S>> for Router<S> {
    fn from(r: ValidatorsRouter<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/validators/countActiveValidators handler
pub trait CountActiveValidatorsInNetworkHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> CountActiveValidatorsInNetworkHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::CountActiveValidatorsInNetworkPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::CountActiveValidatorsInNetworkResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::CountActiveValidatorsInNetworkPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::CountActiveValidatorsInNetworkResponse = res.into();

            response.into()
        })
    }
}

macro_rules! count_active_validators_in_network_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> CountActiveValidatorsInNetworkHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::CountActiveValidatorsInNetworkPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::CountActiveValidatorsInNetworkResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::CountActiveValidatorsInNetworkPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::CountActiveValidatorsInNetworkResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(count_active_validators_in_network_handler);

/// GET /api/v4/{network}/validators/countActiveValidators service
struct CountActiveValidatorsInNetworkService<H, T, S>
where
    H: CountActiveValidatorsInNetworkHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for CountActiveValidatorsInNetworkService<H, T, S>
where
    H: CountActiveValidatorsInNetworkHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> CountActiveValidatorsInNetworkService<H, T, S>
where
    H: CountActiveValidatorsInNetworkHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for CountActiveValidatorsInNetworkService<H, T, S>
where
    H: CountActiveValidatorsInNetworkHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( CountActiveValidatorsInNetworkHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators/owned_by/{ownerAddress}/cost handler
pub trait ValidatorsV4ControllerCostHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ValidatorsV4ControllerCostHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ValidatorsV4ControllerCostPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ValidatorsV4ControllerCostResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerCostPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ValidatorsV4ControllerCostResponse = res.into();

            response.into()
        })
    }
}

macro_rules! validators_v4_controller_cost_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ValidatorsV4ControllerCostHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ValidatorsV4ControllerCostPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ValidatorsV4ControllerCostResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerCostPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ValidatorsV4ControllerCostResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(validators_v4_controller_cost_handler);

/// GET /api/v4/{network}/validators/owned_by/{ownerAddress}/cost service
struct ValidatorsV4ControllerCostService<H, T, S>
where
    H: ValidatorsV4ControllerCostHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ValidatorsV4ControllerCostService<H, T, S>
where
    H: ValidatorsV4ControllerCostHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ValidatorsV4ControllerCostService<H, T, S>
where
    H: ValidatorsV4ControllerCostHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ValidatorsV4ControllerCostService<H, T, S>
where
    H: ValidatorsV4ControllerCostHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ValidatorsV4ControllerCostHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators/in_operator/{operator} handler
pub trait ValidatorsV4ControllerInOperatorHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ValidatorsV4ControllerInOperatorHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ValidatorsV4ControllerInOperatorPath,super::endpoint::ValidatorsV4ControllerInOperatorQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ValidatorsV4ControllerInOperatorResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerInOperatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ValidatorsV4ControllerInOperatorQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ValidatorsV4ControllerInOperatorResponse = res.into();

            response.into()
        })
    }
}

macro_rules! validators_v4_controller_in_operator_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ValidatorsV4ControllerInOperatorHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ValidatorsV4ControllerInOperatorPath,super::endpoint::ValidatorsV4ControllerInOperatorQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ValidatorsV4ControllerInOperatorResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerInOperatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ValidatorsV4ControllerInOperatorQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ValidatorsV4ControllerInOperatorResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(validators_v4_controller_in_operator_handler);

/// GET /api/v4/{network}/validators/in_operator/{operator} service
struct ValidatorsV4ControllerInOperatorService<H, T, S>
where
    H: ValidatorsV4ControllerInOperatorHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ValidatorsV4ControllerInOperatorService<H, T, S>
where
    H: ValidatorsV4ControllerInOperatorHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ValidatorsV4ControllerInOperatorService<H, T, S>
where
    H: ValidatorsV4ControllerInOperatorHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ValidatorsV4ControllerInOperatorService<H, T, S>
where
    H: ValidatorsV4ControllerInOperatorHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ValidatorsV4ControllerInOperatorHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators/incentivized/{validator} handler
pub trait ValidatorsV4ControllerIncentivizedHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ValidatorsV4ControllerIncentivizedHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ValidatorsV4ControllerIncentivizedPath,super::endpoint::ValidatorsV4ControllerIncentivizedQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ValidatorsV4ControllerIncentivizedResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerIncentivizedPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ValidatorsV4ControllerIncentivizedQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ValidatorsV4ControllerIncentivizedResponse = res.into();

            response.into()
        })
    }
}

macro_rules! validators_v4_controller_incentivized_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ValidatorsV4ControllerIncentivizedHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ValidatorsV4ControllerIncentivizedPath,super::endpoint::ValidatorsV4ControllerIncentivizedQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ValidatorsV4ControllerIncentivizedResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerIncentivizedPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ValidatorsV4ControllerIncentivizedQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ValidatorsV4ControllerIncentivizedResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(validators_v4_controller_incentivized_handler);

/// GET /api/v4/{network}/validators/incentivized/{validator} service
struct ValidatorsV4ControllerIncentivizedService<H, T, S>
where
    H: ValidatorsV4ControllerIncentivizedHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ValidatorsV4ControllerIncentivizedService<H, T, S>
where
    H: ValidatorsV4ControllerIncentivizedHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ValidatorsV4ControllerIncentivizedService<H, T, S>
where
    H: ValidatorsV4ControllerIncentivizedHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ValidatorsV4ControllerIncentivizedService<H, T, S>
where
    H: ValidatorsV4ControllerIncentivizedHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ValidatorsV4ControllerIncentivizedHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators/{validator} handler
pub trait ValidatorsV4ControllerValidatorHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ValidatorsV4ControllerValidatorHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ValidatorsV4ControllerValidatorPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ValidatorsV4ControllerValidatorResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerValidatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ValidatorsV4ControllerValidatorResponse = res.into();

            response.into()
        })
    }
}

macro_rules! validators_v4_controller_validator_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ValidatorsV4ControllerValidatorHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ValidatorsV4ControllerValidatorPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ValidatorsV4ControllerValidatorResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerValidatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ValidatorsV4ControllerValidatorResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(validators_v4_controller_validator_handler);

/// GET /api/v4/{network}/validators/{validator} service
struct ValidatorsV4ControllerValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ValidatorsV4ControllerValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ValidatorsV4ControllerValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ValidatorsV4ControllerValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ValidatorsV4ControllerValidatorHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators/isRegisteredValidator/{validator} handler
pub trait ValidatorsV4ControllerIsRegisteredValidatorHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ValidatorsV4ControllerIsRegisteredValidatorHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorResponse = res.into();

            response.into()
        })
    }
}

macro_rules! validators_v4_controller_is_registered_validator_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ValidatorsV4ControllerIsRegisteredValidatorHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ValidatorsV4ControllerIsRegisteredValidatorResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(validators_v4_controller_is_registered_validator_handler);

/// GET /api/v4/{network}/validators/isRegisteredValidator/{validator} service
struct ValidatorsV4ControllerIsRegisteredValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerIsRegisteredValidatorHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ValidatorsV4ControllerIsRegisteredValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerIsRegisteredValidatorHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ValidatorsV4ControllerIsRegisteredValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerIsRegisteredValidatorHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ValidatorsV4ControllerIsRegisteredValidatorService<H, T, S>
where
    H: ValidatorsV4ControllerIsRegisteredValidatorHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ValidatorsV4ControllerIsRegisteredValidatorHandler::call(handler, req, state).await)
        })
    }
}


/// POST /api/v4/{network}/validators/registeredByPublicKeys handler
pub trait ApiV4ValidatorRegisteredByPublicKeyCreateHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ApiV4ValidatorRegisteredByPublicKeyCreateHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreatePath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreateResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreatePath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreateResponse = res.into();

            response.into()
        })
    }
}

macro_rules! api_v4_validator_registered_by_public_key_create_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ApiV4ValidatorRegisteredByPublicKeyCreateHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreatePath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreateResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreatePath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ApiV4ValidatorRegisteredByPublicKeyCreateResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(api_v4_validator_registered_by_public_key_create_handler);

/// POST /api/v4/{network}/validators/registeredByPublicKeys service
struct ApiV4ValidatorRegisteredByPublicKeyCreateService<H, T, S>
where
    H: ApiV4ValidatorRegisteredByPublicKeyCreateHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ApiV4ValidatorRegisteredByPublicKeyCreateService<H, T, S>
where
    H: ApiV4ValidatorRegisteredByPublicKeyCreateHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ApiV4ValidatorRegisteredByPublicKeyCreateService<H, T, S>
where
    H: ApiV4ValidatorRegisteredByPublicKeyCreateHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ApiV4ValidatorRegisteredByPublicKeyCreateService<H, T, S>
where
    H: ApiV4ValidatorRegisteredByPublicKeyCreateHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ApiV4ValidatorRegisteredByPublicKeyCreateHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators handler
pub trait ValidatorsV4ControllerValidatorsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ValidatorsV4ControllerValidatorsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ValidatorsV4ControllerValidatorsPath,super::endpoint::ValidatorsV4ControllerValidatorsQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ValidatorsV4ControllerValidatorsResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerValidatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ValidatorsV4ControllerValidatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::ValidatorsV4ControllerValidatorsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! validators_v4_controller_validators_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ValidatorsV4ControllerValidatorsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ValidatorsV4ControllerValidatorsPath,super::endpoint::ValidatorsV4ControllerValidatorsQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ValidatorsV4ControllerValidatorsResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerValidatorsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::ValidatorsV4ControllerValidatorsQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::ValidatorsV4ControllerValidatorsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(validators_v4_controller_validators_handler);

/// GET /api/v4/{network}/validators service
struct ValidatorsV4ControllerValidatorsService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ValidatorsV4ControllerValidatorsService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ValidatorsV4ControllerValidatorsService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ValidatorsV4ControllerValidatorsService<H, T, S>
where
    H: ValidatorsV4ControllerValidatorsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ValidatorsV4ControllerValidatorsHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch} handler
pub trait ValidatorsV4ControllerDutyCountsHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ValidatorsV4ControllerDutyCountsHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ValidatorsV4ControllerDutyCountsPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ValidatorsV4ControllerDutyCountsResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerDutyCountsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ValidatorsV4ControllerDutyCountsResponse = res.into();

            response.into()
        })
    }
}

macro_rules! validators_v4_controller_duty_counts_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ValidatorsV4ControllerDutyCountsHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ValidatorsV4ControllerDutyCountsPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ValidatorsV4ControllerDutyCountsResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ValidatorsV4ControllerDutyCountsPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ValidatorsV4ControllerDutyCountsResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(validators_v4_controller_duty_counts_handler);

/// GET /api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch} service
struct ValidatorsV4ControllerDutyCountsService<H, T, S>
where
    H: ValidatorsV4ControllerDutyCountsHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ValidatorsV4ControllerDutyCountsService<H, T, S>
where
    H: ValidatorsV4ControllerDutyCountsHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ValidatorsV4ControllerDutyCountsService<H, T, S>
where
    H: ValidatorsV4ControllerDutyCountsHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ValidatorsV4ControllerDutyCountsService<H, T, S>
where
    H: ValidatorsV4ControllerDutyCountsHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ValidatorsV4ControllerDutyCountsHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/validators/validatorsByClusterHash/{clusterHash} handler
pub trait ApiV4ValidatorsValidatorsByClusterHashGetHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ApiV4ValidatorsValidatorsByClusterHashGetHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetResponse = res.into();

            response.into()
        })
    }
}

macro_rules! api_v4_validators_validators_by_cluster_hash_get_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ApiV4ValidatorsValidatorsByClusterHashGetHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::ApiV4ValidatorsValidatorsByClusterHashGetResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(api_v4_validators_validators_by_cluster_hash_get_handler);

/// GET /api/v4/{network}/validators/validatorsByClusterHash/{clusterHash} service
struct ApiV4ValidatorsValidatorsByClusterHashGetService<H, T, S>
where
    H: ApiV4ValidatorsValidatorsByClusterHashGetHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ApiV4ValidatorsValidatorsByClusterHashGetService<H, T, S>
where
    H: ApiV4ValidatorsValidatorsByClusterHashGetHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ApiV4ValidatorsValidatorsByClusterHashGetService<H, T, S>
where
    H: ApiV4ValidatorsValidatorsByClusterHashGetHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ApiV4ValidatorsValidatorsByClusterHashGetService<H, T, S>
where
    H: ApiV4ValidatorsValidatorsByClusterHashGetHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ApiV4ValidatorsValidatorsByClusterHashGetHandler::call(handler, req, state).await)
        })
    }
}


/// POST /api/v4/{network}/validators/validatorsWithdrawCredentials handler
pub trait ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreatePath,super::model::ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse>,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreatePath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

            let res = self(path.0, body.0, ).await;

            let response: super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse = res.into();

            response.into()
        })
    }
}

macro_rules! api_v4_validator_validators_withdraw_credential_create_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreatePath,super::model::ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse>,B: HttpBody + Send + 'static,
            B::Data: Send,
            B::Error: Into<BoxError>,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreatePath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    
                    let req = Request::from_parts(parts, body);let body = match axum::Json::<super::model::ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    let res = self(path.0, body.0, $($ty,)*).await;

                    let response: super::endpoint::ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(api_v4_validator_validators_withdraw_credential_create_handler);

/// POST /api/v4/{network}/validators/validatorsWithdrawCredentials service
struct ApiV4ValidatorValidatorsWithdrawCredentialCreateService<H, T, S>
where
    H: ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for ApiV4ValidatorValidatorsWithdrawCredentialCreateService<H, T, S>
where
    H: ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> ApiV4ValidatorValidatorsWithdrawCredentialCreateService<H, T, S>
where
    H: ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for ApiV4ValidatorValidatorsWithdrawCredentialCreateService<H, T, S>
where
    H: ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( ApiV4ValidatorValidatorsWithdrawCredentialCreateHandler::call(handler, req, state).await)
        })
    }
}



/// V4 router
pub struct V4Router<S = ()> {
    pub(crate) router: Router<S>,
}

impl<S> V4Router<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// GET /api/v4/{network}/accounts
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery) -> AccountV4ControllerListResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().account_v4_controller_list(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery, ...extractors) -> AccountV4ControllerListResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().account_v4_controller_list(handler);
    /// ```
    pub fn account_v4_controller_list<H, T>(mut self, handler: H) -> Self
        where
            H: AccountV4ControllerListHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/accounts", on_service(MethodFilter::GET, AccountV4ControllerListService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/accounts/{ownerAddress}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerByIdPath) -> AccountV4ControllerByIdResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().account_v4_controller_by_id(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::AccountV4ControllerByIdPath, ...extractors) -> AccountV4ControllerByIdResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().account_v4_controller_by_id(handler);
    /// ```
    pub fn account_v4_controller_by_id<H, T>(mut self, handler: H) -> Self
        where
            H: AccountV4ControllerByIdHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/accounts/:ownerAddress", on_service(MethodFilter::GET, AccountV4ControllerByIdService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/duties/{validator}
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery) -> DutiesV4ControllerDutiesResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().duties_v4_controller_duties(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery, ...extractors) -> DutiesV4ControllerDutiesResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().duties_v4_controller_duties(handler);
    /// ```
    pub fn duties_v4_controller_duties<H, T>(mut self, handler: H) -> Self
        where
            H: DutiesV4ControllerDutiesHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/duties/:validator", on_service(MethodFilter::GET, DutiesV4ControllerDutiesService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/health
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::HealthV4ControllerHealthPath) -> HealthV4ControllerHealthResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().health_v4_controller_health(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::HealthV4ControllerHealthPath, ...extractors) -> HealthV4ControllerHealthResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().health_v4_controller_health(handler);
    /// ```
    pub fn health_v4_controller_health<H, T>(mut self, handler: H) -> Self
        where
            H: HealthV4ControllerHealthHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/health", on_service(MethodFilter::GET, HealthV4ControllerHealthService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/incentivization/merkle-tree
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath) -> IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().incentivization_v4_controller_migration_operators_distribution(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath, ...extractors) -> IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().incentivization_v4_controller_migration_operators_distribution(handler);
    /// ```
    pub fn incentivization_v4_controller_migration_operators_distribution<H, T>(mut self, handler: H) -> Self
        where
            H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/incentivization/merkle-tree", on_service(MethodFilter::GET, IncentivizationV4ControllerMigrationOperatorsDistributionService::new(handler)));
        self
    }

    /// GET /api/v4/{network}/search
    ///
    /// # Examples
    ///
    /// ```
    /// async fn handler(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery) -> SearchV4ControllerSearchResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().search_v4_controller_search(handler);
    /// ```
    ///
    /// ```
    /// async fn handler(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery, ...extractors) -> SearchV4ControllerSearchResponse {
    ///     todo!();
    /// }
    /// let router = V4Router::default().search_v4_controller_search(handler);
    /// ```
    pub fn search_v4_controller_search<H, T>(mut self, handler: H) -> Self
        where
            H: SearchV4ControllerSearchHandler<T, S>,
            T: 'static
    {
        self.router = self.router.route("/api/v4/:network/search", on_service(MethodFilter::GET, SearchV4ControllerSearchService::new(handler)));
        self
    }
}

impl<S> Default for V4Router<S> where S: Clone + Send + Sync + 'static  {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<V4Router<S>> for Router<S> {
    fn from(r: V4Router<S>) -> Self {
        r.router
    }
}

/// GET /api/v4/{network}/accounts handler
pub trait AccountV4ControllerListHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> AccountV4ControllerListHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::AccountV4ControllerListResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerListPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::AccountV4ControllerListQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::AccountV4ControllerListResponse = res.into();

            response.into()
        })
    }
}

macro_rules! account_v4_controller_list_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> AccountV4ControllerListHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::AccountV4ControllerListPath,super::endpoint::AccountV4ControllerListQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::AccountV4ControllerListResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerListPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::AccountV4ControllerListQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::AccountV4ControllerListResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(account_v4_controller_list_handler);

/// GET /api/v4/{network}/accounts service
struct AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for AccountV4ControllerListService<H, T, S>
where
    H: AccountV4ControllerListHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( AccountV4ControllerListHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/accounts/{ownerAddress} handler
pub trait AccountV4ControllerByIdHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> AccountV4ControllerByIdHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::AccountV4ControllerByIdPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::AccountV4ControllerByIdResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerByIdPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::AccountV4ControllerByIdResponse = res.into();

            response.into()
        })
    }
}

macro_rules! account_v4_controller_by_id_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> AccountV4ControllerByIdHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::AccountV4ControllerByIdPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::AccountV4ControllerByIdResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::AccountV4ControllerByIdPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::AccountV4ControllerByIdResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(account_v4_controller_by_id_handler);

/// GET /api/v4/{network}/accounts/{ownerAddress} service
struct AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for AccountV4ControllerByIdService<H, T, S>
where
    H: AccountV4ControllerByIdHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( AccountV4ControllerByIdHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/duties/{validator} handler
pub trait DutiesV4ControllerDutiesHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> DutiesV4ControllerDutiesHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::DutiesV4ControllerDutiesResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::DutiesV4ControllerDutiesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::DutiesV4ControllerDutiesQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::DutiesV4ControllerDutiesResponse = res.into();

            response.into()
        })
    }
}

macro_rules! duties_v4_controller_duties_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> DutiesV4ControllerDutiesHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::DutiesV4ControllerDutiesPath,super::endpoint::DutiesV4ControllerDutiesQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::DutiesV4ControllerDutiesResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::DutiesV4ControllerDutiesPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::DutiesV4ControllerDutiesQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::DutiesV4ControllerDutiesResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(duties_v4_controller_duties_handler);

/// GET /api/v4/{network}/duties/{validator} service
struct DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for DutiesV4ControllerDutiesService<H, T, S>
where
    H: DutiesV4ControllerDutiesHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( DutiesV4ControllerDutiesHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/health handler
pub trait HealthV4ControllerHealthHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> HealthV4ControllerHealthHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::HealthV4ControllerHealthPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::HealthV4ControllerHealthResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::HealthV4ControllerHealthPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::HealthV4ControllerHealthResponse = res.into();

            response.into()
        })
    }
}

macro_rules! health_v4_controller_health_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> HealthV4ControllerHealthHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::HealthV4ControllerHealthPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::HealthV4ControllerHealthResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::HealthV4ControllerHealthPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::HealthV4ControllerHealthResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(health_v4_controller_health_handler);

/// GET /api/v4/{network}/health service
struct HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for HealthV4ControllerHealthService<H, T, S>
where
    H: HealthV4ControllerHealthHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( HealthV4ControllerHealthHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/incentivization/merkle-tree handler
pub trait IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> IncentivizationV4ControllerMigrationOperatorsDistributionHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, ).await;

            let response: super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse = res.into();

            response.into()
        })
    }
}

macro_rules! incentivization_v4_controller_migration_operators_distribution_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> IncentivizationV4ControllerMigrationOperatorsDistributionHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, $($ty,)*).await;

                    let response: super::endpoint::IncentivizationV4ControllerMigrationOperatorsDistributionResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(incentivization_v4_controller_migration_operators_distribution_handler);

/// GET /api/v4/{network}/incentivization/merkle-tree service
struct IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for IncentivizationV4ControllerMigrationOperatorsDistributionService<H, T, S>
where
    H: IncentivizationV4ControllerMigrationOperatorsDistributionHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( IncentivizationV4ControllerMigrationOperatorsDistributionHandler::call(handler, req, state).await)
        })
    }
}


/// GET /api/v4/{network}/search handler
pub trait SearchV4ControllerSearchHandler<T, S, B = Body>: Clone + Send + Sized + 'static {
    fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

impl<F, Fut, S, B> SearchV4ControllerSearchHandler<(), S, B> for F
where
    F: FnOnce(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery,) -> Fut + Clone + Send + 'static,
    Fut: Future + Send,
    Fut::Output: Into<super::endpoint::SearchV4ControllerSearchResponse>,
    
    B: Send + 'static,
    
    S: Send + Sync + 'static,
{fn call(
        self,
        req: ERequest,
        state: S
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let state = &state;


                    let path = match axum::extract::Path::<super::endpoint::SearchV4ControllerSearchPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::SearchV4ControllerSearchQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };


            let res = self(path.0, query.0, ).await;

            let response: super::endpoint::SearchV4ControllerSearchResponse = res.into();

            response.into()
        })
    }
}

macro_rules! search_v4_controller_search_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<F, Fut, S, B, $($ty,)*> SearchV4ControllerSearchHandler<($($ty,)*), S, B> for F
        where
            F: FnOnce(super::endpoint::SearchV4ControllerSearchPath,super::endpoint::SearchV4ControllerSearchQuery,$($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future + Send,
            Fut::Output: Into<super::endpoint::SearchV4ControllerSearchResponse>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $( $ty: FromRequestParts<S> + Send,)*
        {
            fn call(self, req: ERequest, state: S) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    
                    let path = match axum::extract::Path::<super::endpoint::SearchV4ControllerSearchPath>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };
                    let query = match axum::extract::Query::<super::endpoint::SearchV4ControllerSearchQuery>::from_request_parts(&mut parts, state).await {
                        Ok(value) => value,
                        Err(err) => return err.into_response(),
                    };

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*


                    

                    let res = self(path.0, query.0, $($ty,)*).await;

                    let response: super::endpoint::SearchV4ControllerSearchResponse = res.into();

                    response.into()
                })
            }
        }
    };
}

all_the_tuples!(search_v4_controller_search_handler);

/// GET /api/v4/{network}/search service
struct SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<H, T, S> Clone for SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S>
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<ERequest> for SearchV4ControllerSearchService<H, T, S>
where
    H: SearchV4ControllerSearchHandler<T, S>,
    S: Send + Sync + 'static {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: ERequest) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            let state = req
                .extensions_mut()
                .remove::<S>()
                .expect("state extension missing. This is a bug in code schema-tools, please file an issue");

            Ok( SearchV4ControllerSearchHandler::call(handler, req, state).await)
        })
    }
}


