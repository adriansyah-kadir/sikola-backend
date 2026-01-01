use crate::app::App;
use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{Validation, decode};
use serde::{Deserialize, Serialize};

pub type Bearer = axum_extra::TypedHeader<
    axum_extra::headers::Authorization<axum_extra::headers::authorization::Bearer>,
>;

pub fn public_key() -> Result<jsonwebtoken::DecodingKey, jsonwebtoken::errors::Error> {
    jsonwebtoken::DecodingKey::from_rsa_components(
        "yAcBu_w5fZCauZv3jkSbek7Z4pX-h7rmhWdxVfHt6azWjz74UmkKzHIhwvPW2DyNUjZ56oRJAIJ9YDKoW37mhn-K8b8-gjrtgh96Fobg3Ga4emxwfiTYsPkR9XXiKZPo-zGdnEJ9he_pfDC-bLwsK0J4sAJUsq9lDO33oiL_u3elQOis5uP4BQL46ne4SceV6zQSMpxnISwK0ayv0Ckn7_4BuB_MMCBlfFTUAnOAS4buQ3o4sjPztCnwBB6A1H-2fTD0R4NbyFO9VCiEyU5JrhVwp5BF3Ska5HS3htm1uRED7oE-utHSu_bL-pGf_LynBl3V8WBOZgOMrcu4cZwaLQ",
        "AQAB",
    )
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JWTClaims {
    pub sub: uuid::Uuid,
    pub aud: String,
    pub preferred_username: String,
}

impl OptionalFromRequestParts<App> for JWTClaims {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Option<Self>, Self::Rejection> {
        <Self as FromRequestParts<App>>::from_request_parts(parts, state)
            .await
            .map(Some)
            .or_else(|_| {
                Ok(None)
            })
    }
}

impl FromRequestParts<App> for JWTClaims {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let decode_key = public_key()
            .map_err(|err| {
                tracing::error!("{:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "invalid jwt decode key").into_response()
            })?;
        
        let jwt: Bearer = <Bearer as FromRequestParts<App>>::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&["account"]);

        decode::<JWTClaims>(jwt.token(), &decode_key, &validation)
            .map(|token| token.claims)
            .map_err(|err| {
                tracing::error!("jwt decode error {:?}", err.kind());
                (StatusCode::UNAUTHORIZED, "Failed to verify token").into_response()
            })
    }
}
