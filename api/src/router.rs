use crate::handlers;
use crate::middleware::AuthLayer;
use crate::state::AppState;
use axum::Router;
use axum::routing::get;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    let api = Router::new()
        .route(
            "/resources",
            get(handlers::list_resources).post(handlers::create_resource),
        )
        .route(
            "/resources/{id}",
            get(handlers::get_resource)
                .put(handlers::update_resource)
                .delete(handlers::delete_resource),
        )
        .layer(AuthLayer::new(state.config.auth_enabled));
    Router::new()
        .route("/health", get(handlers::health_check))
        .nest("/api/v1", api)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}
