use axum::{http::HeaderMap, response::sse::KeepAlive};
use keycloak::models::UserRepresentation;
use reqwest::{Client, StatusCode};
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
        pub access_token: String,
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

fn safe_user_data(v: keycloak::models::UserRepresentation) -> keycloak::models::UserRepresentation {
    keycloak::models::UserRepresentation {
        id: v.id,
        username: v.username,
        first_name: v.first_name,
        last_name: v.last_name,
        email: v.email,
        email_verified: v.email_verified,
        attributes: v.attributes,
        enabled: v.enabled,
        ..Default::default()
    }
}

#[derive(Debug)]
pub enum RequestError<T> {
    Other(Box<dyn std::error::Error>),
    Keycloak(keycloak::apis::Error<T>),
}

pub async fn realm_user(
    realm: &str,
    user_id: &str,
) -> Result<
    Option<UserRepresentation>,
    RequestError<keycloak::apis::users_api::AdminRealmsRealmUsersUserIdGetError>,
> {
    let config = get_config().await.map_err(RequestError::Other)?;
    keycloak::apis::users_api::admin_realms_realm_users_user_id_get(&config, realm, user_id, None)
        .await
        .map(|v| Some(safe_user_data(v)))
        .or_else(|err| match err {
            keycloak::apis::Error::ResponseError(response_content) => match response_content.status
            {
                StatusCode::NOT_FOUND => Ok(None),
                _ => Err(RequestError::Keycloak(
                    keycloak::apis::Error::ResponseError(response_content),
                )),
            },
            _ => Err(RequestError::Keycloak(err)),
        })
}

pub async fn realm_users(
    realm: &str,
    search: Option<keycloak::models::UserRepresentation>,
) -> Result<
    Vec<keycloak::models::UserRepresentation>,
    RequestError<keycloak::apis::users_api::AdminRealmsRealmUsersGetError>,
> {
    let config = get_config().await.map_err(RequestError::Other)?;
    keycloak::apis::users_api::admin_realms_realm_users_get(
        &config,
        realm,
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
        None,
        search.clone().and_then(|v| v.email).as_deref(),
        search.clone().and_then(|v| v.email_verified),
        search.clone().and_then(|v| v.enabled),
        None,
        None,
        search.clone().and_then(|v| v.first_name).as_deref(),
        None,
        None,
        search.clone().and_then(|v| v.last_name).as_deref(),
        None,
        None,
        None,
        search.clone().and_then(|v| v.username).as_deref(),
    )
    .await
    .map(|v| v.iter().map(|v| v.to_owned()).map(safe_user_data).collect())
    .map_err(RequestError::Keycloak)
}
