use actix_web::{HttpResponse, post, web};

use crate::domain::order::Order;
use crate::infraestructure::http::structs::GenericError;
use crate::infraestructure::soap::request::SoapRequest;

#[post("/process")]
async fn process_json(
    request: web::Json<Vec<Order>>,
) -> Result<HttpResponse, GenericError> {
    let soap_url = std::env::var("WSDL_URL");
    if soap_url.is_err() {
        return Err(
            GenericError {
                code: 422,
                cause: "WSDL_URL environment variable is not set".to_string(),
            }
        );
    }

    let result = SoapRequest::new(
        soap_url.unwrap(),
        request.into_inner(),
    ).send().await;

    if result.is_err() {
        return Err(
            GenericError::new(500, result.err().unwrap().to_string()));
    }

    Ok(
        HttpResponse::Ok()
            .content_type("text/xml")
            .body(result.unwrap()),
    )
}