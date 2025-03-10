use actix_http::h1::Payload;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::BytesMut,
    Error,
};

use futures_util::StreamExt;
use hmac::{Hmac, Mac};

use crate::configuration::CONFIGURATION;

pub async fn hmac_middleware(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let hmac_header = req
        .headers()
        .get("X-HMAC-Signature")
        .and_then(|v| v.to_str().ok());

    if hmac_header.is_none() || hmac_header.unwrap().is_empty() {
        return Err(actix_web::error::ErrorForbidden(""));
    }
    let hmac = hex::decode(hmac_header.unwrap())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid HMAC header"))?;

    let (_, pay) = req.parts_mut();

    let mut body = BytesMut::new();
    let mut mac = Hmac::<sha2::Sha256>::new_from_slice(&CONFIGURATION.get_hmac_key()).unwrap();

    while let Some(chunk) = pay.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
        mac.update(&chunk);
    }

    mac.verify_slice(&hmac)
        .map_err(|_| actix_web::error::ErrorForbidden(""))?;

    let (_, mut payload) = Payload::create(true);
    payload.unread_data(body.into());
    req.set_payload(payload.into());

    next.call(req).await
}
