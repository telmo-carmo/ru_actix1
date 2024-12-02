use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
//use actix_web_httpauth::extractors::bearer::BearerAuth;
//use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

const JWT_SECRET_KEY : &str = "my_very_secret_key";

/*
async fn validator_jwt(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let decoding_key = DecodingKey::from_secret(JWT_SECRET_KEY.as_ref());

    match decode::<Claims>(&token, &decoding_key, &Validation::new(Algorithm::HS256)) {
        Ok(_) => Ok(req),
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}
*/

// pub fn jwt_middleware() -> HttpAuthentication {
//     HttpAuthentication::bearer(validator_jwt)
// }

pub fn generate_jwt(sub: &str, company: &str, exp: usize) -> Result<String, Error> {
    let claims = Claims {
        sub: sub.to_owned(),
        company: company.to_owned(),
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()))
        .map_err(|_| actix_web::error::ErrorInternalServerError("Token generation error"))
}