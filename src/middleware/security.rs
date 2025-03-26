use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::Error;
use crate::security::headers::{
    XFrameOptionsBuilder,
    CSPBuilder,
    FeaturePolicyBuilder,
    FeaturePolicy
};
use futures::future::{ready, Ready, LocalBoxFuture};pub struct SecurityHeaders;
impl SecurityHeaders {
    pub fn new() -> Self {
        SecurityHeaders
    }
}

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SecurityHeadersMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHeadersMiddleware { service }))
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            
            // Add X-Frame-Options header
            let x_frame_options = XFrameOptionsBuilder::new().build();
            res.headers_mut().insert(
                HeaderName::from_static("x-frame-options"),
                HeaderValue::from_str(&x_frame_options).unwrap()
            );

            // Add CSP header
            let csp = CSPBuilder::new()
                .default_src(vec!["'self'"])
                .script_src(vec!["'self'"])
                .style_src(vec!["'self'"])
                .img_src(vec!["'self'"])
                .build();
            res.headers_mut().insert(
                HeaderName::from_static("content-security-policy"),
                HeaderValue::from_str(&csp).unwrap()
            );

            // Add Feature-Policy header
            let mut feature_policy = FeaturePolicyBuilder::new();
            feature_policy
                .add_feature("camera", FeaturePolicy::None)
                .add_feature("microphone", FeaturePolicy::Self_)
                .add_feature("payment", FeaturePolicy::Origins(vec!["https://payment.example.com".to_string()]));
            
            res.headers_mut().insert(
                HeaderName::from_static("feature-policy"),
                HeaderValue::from_str(&feature_policy.build()).unwrap()
            );
            
            Ok(res)
        })    }}