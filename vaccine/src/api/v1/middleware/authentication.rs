use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ok, Future, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::rc::Rc;

use crate::api::v1::config;
use crate::api::v1::models::response_model::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub iat: usize,
    pub exp: usize,
    pub nbf: usize,
    pub jti: String,
    pub sub: String,
    pub prv: String,
}

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware { 
            service: Rc::new(service) 
        })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
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
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract token from Authorization header
            let token = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_str| {
                    if auth_str.starts_with("Bearer ") {
                        Some(auth_str[7..].trim())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| -> Error {
                    let error_response = HttpResponse::Unauthorized()
                        .json(ApiResponse::new("Missing or invalid authorization header"));
                    actix_web::error::InternalError::from_response(
                        "Auth error",
                        error_response,
                    )
                    .into()
                })?;

            // Validate JWT and extract claims
            let claims = validate_token(token)
                .map_err(|_| -> Error {
                    let error_response = HttpResponse::Unauthorized()
                        .json(ApiResponse::new("Invalid token"));
                    actix_web::error::InternalError::from_response(
                        "Invalid token",
                        error_response,
                    )
                    .into()
                })?;

            // Store claims in request extensions for later use
            req.extensions_mut().insert(claims);

            // Continue with the request
            service.call(req).await
        })
    }
}

fn validate_token(token: &str) -> Result<Claims, Error> {
    // Remove "Bearer " prefix if it exists
    let token = token
        .trim()
        .strip_prefix("Bearer ")
        .unwrap_or(token)
        .trim();
    
    
    let secret = config::config::get_jwt_secret_key();    
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(config::config::get_jwt_algorithm());
    
    // Configure validation to check these claims
    validation.validate_exp = true;  // Validate expiration time
    validation.validate_nbf = true;  // Validate not before time

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|token_data| token_data.claims)
        .map_err(|e| {
            println!("Token validation error: {:?}", e);  // Add this for debugging
            let error_response = HttpResponse::Unauthorized()
                .json(ApiResponse::new("Invalid token"));
            actix_web::error::InternalError::from_response(
                "Invalid token",
                error_response,
            )
            .into()
        })
}
