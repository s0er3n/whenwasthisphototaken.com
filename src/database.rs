use std::{env, str::FromStr};

use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    types::Uuid,
    Pool, Postgres,
};

use crate::{auth::User, ImageForm};

#[derive(Clone)]
pub struct DataBase {
    pool: Pool<Postgres>,
}

#[derive(sqlx::FromRow)]
struct UserDB {
    name: String,
    provider: String,
    provider_id: String,
    pfp: Option<String>,
}

impl From<User> for UserDB {
    fn from(value: User) -> Self {
        UserDB {
            name: value.display_name,
            // hardcoded
            provider: "twitch".into(),
            provider_id: value.id,
            pfp: Some(value.profile_image_url),
        }
    }
}
#[derive(sqlx::FromRow)]
pub struct Image {
    image: Vec<u8>,
    tags: String,
    description: String,
    year: i32,
    user_id: sqlx::types::Uuid,
}

impl Image {
    pub fn from_form(value: ImageForm, id: String) -> Self {
        Image {
            // TODO: expensive clone i think
            image: value.files.first().expect("idk").data.clone().into(),
            tags: value.tags.to_string(),
            description: value.description.to_string(),
            year: value.year.to_owned() as i32,
            user_id: Uuid::from_str(&id).expect("should always be uuid"),
        }
    }
}

impl DataBase {
    pub async fn new() -> Self {
        // TODO: think if i really need a pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").expect("db url has to exist"))
            .await
            .expect("db connection to work");
        Self { pool }
    }
    pub async fn insert_user(&self, user: User) -> anyhow::Result<String> {
        let user: UserDB = user.into();
        // FIXME: this should also upsert
        let result = sqlx::query!(
            r#"INSERT INTO "user" (id, name, provider, provider_id, pfp) VALUES (gen_random_uuid(), $1, $2, $3, $4)
            ON CONFLICT (provider_id) DO UPDATE SET name = excluded.name, pfp = excluded.pfp
            RETURNING id"#,
            user.name,
            user.provider,
            format!("{}:{}",user.provider,user.provider_id),
            user.pfp
        )
        .fetch_one(&self.pool)
        .await;
        Ok(result?.id.to_string())
    }

    pub async fn insert_image(&self, image: Image) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO "image" (id,image, tags, description, year, user_id, approved) VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, FALSE)"#,
            image.image,
            image.tags,
            image.description,
            image.year,
            image.user_id
        )
        .execute(&self.pool)
        .await
    }
    pub async fn get_image_bytes_by_id(&self, id: &str) -> anyhow::Result<Option<Vec<u8>>> {
        let id = Uuid::from_str(&id)?;
        let result = sqlx::query!(r#"SELECT image FROM "image" WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => Ok(Some(row.image)),
            None => Ok(None),
        }
    }
}
