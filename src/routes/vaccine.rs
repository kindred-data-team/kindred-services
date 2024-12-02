use axum::{routing::{post, get, put, delete}, Router};
use crate::api::vaccine::{create_vaccine, get_vaccine, get_vaccines, update_vaccines, delete_vaccines};

pub fn vaccine_routes() -> Router {
    Router::new()
        .route("/vaccines", post(create_vaccine).get(get_vaccines))
        .route("/vaccines/:id", get(get_vaccine).put(update_vaccines).delete(delete_vaccines))
}