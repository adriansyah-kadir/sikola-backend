use axum::http::HeaderMap;
use reqwest::Client;
use std::{collections::HashMap, env};

pub async fn client_token() -> Result<String, Box<dyn std::error::Error>> {
    let kc_url = env::var("KC_URL")?.trim_end_matches('/').to_owned();
    let realm = env::var("KC_REALM")?;
    let client_id = env::var("KC_CLIENT_ID")?;
    let client_secret = env::var("KC_CLIENT_SECRET")?;
    let token_url = format!("{kc_url}/realms/{realm}/protocol/openid-connect/token");

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", &client_id);
    params.insert("client_secret", &client_secret);

    #[derive(serde::Deserialize)]
    struct Response {
        pub access_token: String
    }

    let client = Client::new();
    let response = client
        .post(token_url)
        .form(&params)
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(response.access_token.clone())
}

pub async fn get_config()
-> Result<keycloak::apis::configuration::Configuration, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let base_path = env::var("KC_URL")?.trim_end_matches('/').to_owned();
    let token = client_token().await?;

    headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    Ok(keycloak::apis::configuration::Configuration {
        base_path,
        client,
        ..Default::default()
    })
}

#[derive(Debug)]
pub enum RequestError<T> {
    Other(Box<dyn std::error::Error>),
    Keycloak(keycloak::apis::Error<T>),
}

pub async fn realm_users(
    realm: &str,
) -> Result<
    Vec<keycloak::models::UserRepresentation>,
    RequestError<keycloak::apis::users_api::AdminRealmsRealmUsersGetError>,
> {
    let config = get_config().await.map_err(RequestError::Other)?;
    keycloak::apis::users_api::admin_realms_realm_users_get(
        &config, realm,
        // brief_representation,
        // email,
        // email_verified,
        // enabled,
        // exact,
        // first,
        // first_name,
        // idp_alias,
        // idp_user_id,
        // last_name,
        // max,
        // q,
        // search,
        // username,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    )
    .await
    .map_err(RequestError::Keycloak)
}
