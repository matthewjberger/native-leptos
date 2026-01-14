use crate::handlers;
use crate::middleware::AuthLayer;
use crate::state::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new().route("/health", get(handlers::health_check));

    let protected_routes = Router::new()
        .route("/resources", get(handlers::list_resources))
        .route("/resources", post(handlers::create_resource))
        .route("/resources/{id}", get(handlers::get_resource))
        .route("/resources/{id}", put(handlers::update_resource))
        .route("/resources/{id}", delete(handlers::delete_resource))
        .layer(AuthLayer::new(state.config.auth_enabled));

    Router::new()
        .merge(public_routes)
        .nest("/api/v1", protected_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        )
        .with_state(state)
}
