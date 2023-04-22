use std::env;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::database::DataBase;

#[derive(Clone)]
pub struct Auth {
    client_id: String,
    client_secret: String,
    db: DataBase,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersData {
    pub data: Vec<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub login: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "broadcaster_type")]
    pub broadcaster_type: String,
    pub description: String,
    #[serde(rename = "profile_image_url")]
    pub profile_image_url: String,
    #[serde(rename = "offline_image_url")]
    pub offline_image_url: String,
    #[serde(rename = "view_count")]
    pub view_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

pub struct SessionData {
    pub channel: String,
    pub id: String,
}

impl Auth {
    pub async fn check_code(&self, code: String) -> Result<SessionData> {
        let client = reqwest::Client::new();
        let client_id = self.client_id.clone();
        let client_secret = self.client_secret.clone();
        let code = code.to_owned();
        let params = [
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code),
            ("grant_type", "authorization_code".into()),
            ("redirect_uri", "http://localhost:3000/".into()),
            ("scope", "".into()),
        ];
        let res: reqwest::Response = client
            .post("https://id.twitch.tv/oauth2/token")
            .form(&params)
            .send()
            .await?;

        if res.status() == 200 {
            let res: Token = serde_json::from_str(&res.text().await?)?;
            let user_res = client
                .get("https://api.twitch.tv/helix/users")
                .header("Authorization", format!("Bearer {}", res.access_token))
                .header("Client-Id", self.client_id.clone())
                .send()
                .await?;
            if user_res.status() == 200 {
                let users: UsersData = serde_json::from_str(&user_res.text().await?)?;
                let user = users.data.first().context("no user")?;
                let id = self.db.insert_user(user.clone()).await?;
                return Ok(SessionData {
                    channel: user.login.clone(),
                    id,
                });
            }
        };

        bail!("could not be authenticated")
    }

    pub fn new(db: DataBase) -> Self {
        Self {
            db,
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID Missing"),
            client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET Missing"),
        }
    }
}
