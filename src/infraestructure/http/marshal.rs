use actix_web::{
    error,
    error::{JsonPayloadError, QueryPayloadError},
    HttpRequest, HttpResponse,
};
use actix_web::http::StatusCode;

use super::structs::GenericError;

pub fn json_handler(err: JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let detail = err.to_string();

    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(GenericError {
            code: StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16(),
            cause: detail,
        }),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(GenericError {
                code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                cause: detail,
            })
        }
        _ => HttpResponse::BadRequest().json(GenericError {
            code: StatusCode::BAD_REQUEST.as_u16(),
            cause: detail,
        }),
    };

    error::InternalError::from_response(err, resp).into()
}

pub fn query_handler(err: QueryPayloadError, _req: &HttpRequest) -> error::Error {
    let detail = err.to_string();

    let resp = match &err {
        QueryPayloadError::Deserialize(..) => {
            HttpResponse::UnprocessableEntity().json(GenericError {
                code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                cause: detail,
            })
        }
        _ => HttpResponse::BadRequest().json(GenericError {
            code: StatusCode::BAD_REQUEST.as_u16(),
            cause: detail,
        }),
    };

    error::InternalError::from_response(err, resp).into()
}