use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthLayer {
    enabled: bool,
}

impl AuthLayer {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            enabled: self.enabled,
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    enabled: bool,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(context)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let enabled = self.enabled;
        let mut inner = self.inner.clone();

        Box::pin(async move {
            if !enabled {
                request.extensions_mut().insert(AuthContext::anonymous());
                return inner.call(request).await;
            }

            let auth_header = request
                .headers()
                .get(http::header::AUTHORIZATION)
                .and_then(|header| header.to_str().ok());

            match authenticate(auth_header).await {
                Some(auth_context) => {
                    request.extensions_mut().insert(auth_context);
                    inner.call(request).await
                }
                None => Ok(StatusCode::UNAUTHORIZED.into_response()),
            }
        })
    }
}

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: Option<String>,
    pub is_authenticated: bool,
}

impl AuthContext {
    pub fn anonymous() -> Self {
        Self {
            user_id: None,
            is_authenticated: false,
        }
    }
}

async fn authenticate(_auth_header: Option<&str>) -> Option<AuthContext> {
    Some(AuthContext::anonymous())
}
