pub mod handler;

use std::sync::Arc;

fn router(ctx: Arc<AppContext>) -> Router {

    Router::new()
        .nest("/api/v1/authentication", handler::router())
        .layer(Extension(handler::AppContext::new()))
    
}
