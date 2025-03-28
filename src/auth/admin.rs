use actix_web::dev::{Service, Transform, ServiceRequest, ServiceResponse};
use actix_web::{Error, web, HttpResponse};
use actix_session::{Session, SessionExt};
use serde::Deserialize;
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Deserialize)]
pub struct AdminLoginData {
    username: String,
    password: String,
}

pub struct AdminGuard;
pub struct AdminGuardMiddleware<S> {
    service: S,
}

impl AdminGuard {
    pub fn new() -> Self {
        AdminGuard
    }
}

impl<S, B> Transform<S, ServiceRequest> for AdminGuard 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminGuardMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AdminGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let fut = self.service.call(req);
        
        Box::pin(async move {
            if session.get::<bool>("is_admin")?.unwrap_or(false) {
                let res = fut.await?;
                Ok(res)
            } else {
                Err(Error::from(actix_web::error::ErrorUnauthorized("Admin access required")))
            }
        })
    }
}

pub async fn admin_login(
    credentials: web::Json<AdminLoginData>,
    session: Session,
) -> Result<HttpResponse, Error> {
    if credentials.username == "admin" && credentials.password == "secure_admin" {
        session.insert("is_admin", true)?;
        Ok(HttpResponse::Ok().json("Admin login successful"))
    } else {
        Ok(HttpResponse::Unauthorized().json("Invalid credentials"))
    }
}