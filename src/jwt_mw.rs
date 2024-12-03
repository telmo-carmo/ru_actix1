use actix_web::Error;
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use log::warn;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

const JWT_SECRET_KEY : &str = "my_very_secret_key";

pub fn generate_jwt(sub: &str, role: &str, exp: usize) -> Result<String, Error> {
    let claims = Claims {
        sub: sub.to_owned(),
        role: role.to_owned(),
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()))
        .map_err(|_| actix_web::error::ErrorInternalServerError("Token generation error"))
}


pub fn validate_jwt(hdr_auth: &str) -> Result<Claims, Error> {
    let token = hdr_auth.strip_prefix("Bearer ")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let decoding_key = DecodingKey::from_secret(JWT_SECRET_KEY.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(&token, &decoding_key, &validation)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid JWT token"))?;

    let timestamp_i64 = token_data.claims.exp as i64;

    // Create a NaiveDateTime from the timestamp
    let naive_datetime = DateTime::from_timestamp(timestamp_i64, 0);
    // Create a DateTime with UTC timezone
    let datetime: DateTime<Utc> = naive_datetime.unwrap_or_else(||{ Utc::now()});
    let expires_datetime = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    
    warn!("JWT expires at {}",expires_datetime);
    Ok(token_data.claims)
}