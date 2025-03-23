use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use actix_session::SessionExt;
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;

pub struct SessionGuard;

impl<S, B> Transform<S, ServiceRequest> for SessionGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SessionGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SessionGuardMiddleware { service })
    }
}

pub struct SessionGuardMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SessionGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        
        if req.path().starts_with("/api/") {
            if session.get::<String>("user_id").unwrap().is_none() {
                return Box::pin(async { Err(Error::from(actix_web::error::ErrorUnauthorized("Unauthorized"))) });
            }
        }
        
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}