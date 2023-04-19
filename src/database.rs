use std::env;

use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    Pool, Postgres,
};

use crate::auth::User;

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
struct Image {
    image: Vec<u8>,
    tags: String,
    description: String,
    year: i32,
    user_id: sqlx::types::Uuid,
    approved: bool,
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
    pub async fn insert_user(&self, user: User) -> Result<PgQueryResult, sqlx::Error> {
        let user: UserDB = user.into();
        // FIXME: this should also upsert
        sqlx::query!(
            r#"INSERT INTO "user" (id,name, provider, provider_id, pfp) VALUES (gen_random_uuid(), $1, $2, $3, $4)"#,
            user.name,
            user.provider,
            format!("{}:{}",user.provider,user.provider_id),
            user.pfp
        )
        .execute(&self.pool)
        .await
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
}
