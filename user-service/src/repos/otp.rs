use crate::{
    config::db::pool,
    entities::otp::{Otp, OtpCreate, OtpMedia, OtpType, OtpUpdate},
};
use chrono::Utc;

use super::traits::Crud;

pub struct OtpRepo;

impl OtpRepo {
    pub async fn get_by_key_username(key: &str, username: &str) -> Result<Otp, String> {
        let db = pool().await.clone();
        let res = sqlx::query!(
            "
            SELECT o.* FROM otps as o
            INNER JOIN users as u ON o.user_id = u.id
            WHERE o.key = $1 AND u.username = $2
        ",
            key,
            username
        )
        .fetch_one(&db)
        .await;
        match res {
            Ok(rec) => Ok(Otp {
                id: rec.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_used: rec.is_used,
                key: rec.key,
                user_id: rec.user_id,
                r#type: OtpType::from_int(rec.r#type),
                media: OtpMedia::from_int(rec.media),
            }),
            Err(_) => Err(String::from("otp not found")),
        }
    }
}

impl Crud for OtpRepo {
    type ReadModel = Otp;
    type CreateModel = OtpCreate;
    type UpdateModel = OtpUpdate;

    async fn get_by_id(id: uuid::Uuid) -> Result<Self::ReadModel, String> {
        let db = pool().await.clone();
        let res = sqlx::query!("SELECT * FROM otps WHERE id=$1", id)
            .fetch_one(&db)
            .await;
        match res {
            Ok(rec) => Ok(Otp {
                id: rec.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_used: rec.is_used,
                key: rec.key,
                user_id: rec.user_id,
                r#type: OtpType::from_int(rec.r#type),
                media: OtpMedia::from_int(rec.media),
            }),
            Err(_) => Err(String::from("otp not found")),
        }
    }

    async fn create(data: Self::CreateModel) -> Result<Self::ReadModel, String> {
        let db = pool().await.clone();
        let res = sqlx::query!(
            "
            INSERT INTO otps (
                created_at,
                updated_at,
                key,
                is_used,
                user_id,
                type,
                media
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, created_at, updated_at
        ",
            Utc::now(),
            Utc::now(),
            data.key,
            data.is_used,
            data.user_id,
            OtpType::to_int(data.r#type),
            OtpMedia::to_int(data.media)
        )
        .fetch_one(&db)
        .await;

        match res {
            Ok(rec) => Ok(Otp {
                id: rec.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_used: data.is_used,
                key: data.key,
                user_id: data.user_id,
                r#type: data.r#type,
                media: data.media,
            }),
            Err(_) => Err(String::from("cannot create the otp")),
        }
    }

    async fn update(data: Self::UpdateModel) -> Result<Self::ReadModel, String> {
        let db = pool().await.clone();
        let res = sqlx::query!(
            "UPDATE otps SET
            updated_at = $1,
            key = $2,
            is_used = $3,
            user_id = $4,
            type = $5,
            media = $6
            WHERE id = $7 RETURNING created_at, updated_at",
            Utc::now(),
            data.key,
            data.is_used,
            data.user_id,
            OtpType::to_int(data.r#type),
            OtpMedia::to_int(data.media),
            data.id,
        )
        .fetch_one(&db)
        .await;
        match res {
            Ok(rec) => Ok(Otp {
                id: data.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_used: data.is_used,
                key: data.key,
                user_id: data.user_id,
                r#type: data.r#type,
                media: data.media,
            }),
            Err(_) => Err(String::from("unable to update the otp")),
        }
    }

    async fn delete(id: uuid::Uuid) -> Result<uuid::Uuid, String> {
        let db = pool().await.clone();
        let res = sqlx::query!("DELETE FROM otps WHERE id = $1", id)
            .execute(&db)
            .await;
        match res {
            Ok(_) => Ok(id),
            Err(_) => Err(String::from("unable to delete the otp")),
        }
    }
}
