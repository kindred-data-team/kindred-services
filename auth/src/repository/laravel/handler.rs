use std::str::FromStr;
use actix_web::HttpRequest;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use scraper::{Html, Selector};

use crate::{config::config::get_laravel_url, models::response::ErrorResponse};

pub async fn handle_laravel_request(token: Option<String>, req: HttpRequest, body: Value) -> Result<String, ErrorResponse>{
    let client = reqwest::Client::new();
    let laravel_url = get_laravel_url();

    let method_str = req.method().as_str();
    let method = reqwest::Method::from_str(method_str).unwrap();

    let mut request_builder = client.request(method, format!("{}{}", laravel_url, req.path()));

    if let Some(t) = token {
        request_builder = request_builder.header(AUTHORIZATION, format!("Bearer {}", t));
    }

    let resp = request_builder
        .json(&body)
        .send()
        .await
        .map_err(|e| ErrorResponse {
            status: 500,
            message: format!("Encountered an error going to the downstream provider: {:?}", e.to_string()),
        })?;

    let resp_status = &resp.status().as_u16();
    if resp.status().is_client_error() || resp.status().is_server_error() {
        // Attempt to parse the error response body as JSON
        let error_body = resp.text().await.map_err(|_| ErrorResponse {
            status: 500,
            message: "Encountered an error while parsing the resposne from the downstream provider.".to_string()
        })?;
        
        let error_json: Result<Value, _> = serde_json::from_str(&error_body);

        match error_json {
            Ok(json) => {
                // If the error body is valid JSON, return it
                return Err(ErrorResponse {
                    status: *resp_status,
                    message: format!("Downstream error: {}", json)
                    }
                );
            }
            Err(_) => {
                let document = Html::parse_document(&error_body);

                let error_message_selector = Selector::parse(".ml-4.text-lg.text-gray-500.uppercase.tracking-wider").unwrap();
                let error_message = document.select(&error_message_selector).next().map(|e| e.text().collect::<String>());
                return Err(
                    ErrorResponse {
                        status: *resp_status,
                        message:  format!("Downstream error: {}",  error_message.unwrap_or("An unknown error occurred".to_string()).trim())
                    }
                );
            }
        }
    } else if *resp_status == 204 {
        let json_resp = serde_json::json!({
            "message": "Success."
        });
        return Ok(json_resp.to_string());
    }
    let json_resp = resp.text().await.map_err(|_| ErrorResponse {
        status: 500,
        message: "Encountered an error while parsing the resposne from the downstream provider.".to_string()
    })?;
    Ok(json_resp)
}