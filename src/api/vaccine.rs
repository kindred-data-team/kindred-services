use crate::models::vaccine::Vaccine;
use crate::repository::vaccine::{insert_vaccine, get_vaccine_by_id, get_all_vaccines, update_vaccine, delete_vaccine};
use axum::{Json, extract::Path, http::StatusCode};
use crate::models::response::ApiResponse;


pub async fn create_vaccine(Json(vaccine): Json<Vaccine>) -> Result<Json<ApiResponse>, StatusCode> {
    // Await the future and then map errors
    insert_vaccine(&vaccine)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Success response
    Ok(Json(ApiResponse::new("Vaccine created successfully.")))
}

pub async fn get_vaccine(Path(id): Path<i32>) -> Result<Json<Vaccine>, StatusCode> {
    // Await the future and then map errors
    get_vaccine_by_id(id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn get_vaccines() -> Result<Json<Vec<Vaccine>>, StatusCode> {
    // Await the future and then map errors
    get_all_vaccines()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_vaccines(Path(id): Path<i32>, Json(vaccine): Json<Vaccine>) -> Result<Json<ApiResponse>, StatusCode> {
    // Await the future and then map errors
    update_vaccine(id, &vaccine)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Success response
    Ok(Json(ApiResponse::new("Vaccine updated successfully.")))
}

pub async fn delete_vaccines(Path(id): Path<i32>) -> Result<Json<ApiResponse>, StatusCode> {
    // Await the future and then map errors
    delete_vaccine(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Success response
    Ok(Json(ApiResponse::new("Vaccine deleted successfully.")))
}