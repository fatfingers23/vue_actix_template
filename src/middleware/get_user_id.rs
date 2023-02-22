use actix_session::SessionExt;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use std::future::{ready, Ready};

use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct GetUserId;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for GetUserId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = GetUserIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(GetUserIdMiddleware { service }))
    }
}

pub struct GetUserIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for GetUserIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let user_key = "user_id";
        match req.get_session().get::<Uuid>(user_key) {
            Ok(possible_id) => {
                if let Some(id) = possible_id {
                    println!("existing SESSION value: {}", id)
                } else {
                    let new_user_id = Uuid::new_v4();
                    println!("new SESSION value: {}", new_user_id);
                    req.get_session()
                        .insert(user_key, new_user_id)
                        .expect("Failed to insert session value");
                }
            }
            Err(e) => {
                return Box::pin(async move { Err(Error::from(e)) });
            }
        };

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
