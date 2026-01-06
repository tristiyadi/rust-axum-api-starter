use jsonwebtoken::{
    encode, decode, Header, EncodingKey, DecodingKey,
    Validation, errors::Error as JwtError
};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: i64,  // i64 untuk user_id
    pub exp: usize,
}

// fungsi untuk menghasilkan token JWT
pub fn generate_token(user_id: i64) -> Result<String, JwtError> {

    // atur waktu kedaluwarsa token (24 jam)
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    
    // buat klaim token
    encode(
        &Header::default(),
        &Claims { sub: user_id, exp },
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string())
                .as_ref()
        ),
    )
}

// fungsi untuk memverifikasi token JWT
pub fn verify_token(token: &str) -> Result<Claims, JwtError> {

    // decode token dan verifikasi
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string())
                .as_ref()
        ),
        &Validation::default(),
    )?;
    
    // kembalikan klaim token
    Ok(token_data.claims)
}
