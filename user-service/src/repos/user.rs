use crate::{
    config::db::pool,
    entities::user::{User, UserCreate, UserUnsafe, UserUpdate, UserUpdateSafe},
};

use super::traits::Crud;
use chrono::Utc;

pub struct UserRepo;

impl UserRepo {
    pub async fn get_by_username(username: &str) -> Result<UserUnsafe, String> {
        let db = pool().await.clone();
        let res = sqlx::query!("SELECT * FROM users WHERE username=$1", username)
            .fetch_one(&db)
            .await;
        match res {
            Ok(rec) => Ok(UserUnsafe {
                id: rec.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                username: rec.username,
                email: rec.email,
                phone: rec.phone,
                email_verified: rec.email_verified,
                phone_verified: rec.phone_verified,
                is_admin: rec.is_admin,
                password: rec.password,
            }),
            Err(_) => Err(String::from("user not found")),
        }
    }

    pub async fn update_safe(data: UserUpdateSafe) -> Result<User, String> {
        let db = pool().await.clone();
        let res = sqlx::query!(
            "UPDATE users SET
            updated_at = $1,
            username = $2,
            phone = $3,
            email = $4,
            phone_verified = $5,
            email_verified = $6,
            is_admin = $7
            WHERE id = $8 RETURNING created_at, updated_at",
            Utc::now(),
            data.username,
            data.phone,
            data.email,
            data.phone_verified,
            data.email_verified,
            data.is_admin,
            data.id,
        )
        .fetch_one(&db)
        .await;
        match res {
            Ok(rec) => Ok(User {
                id: data.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                email: data.email,
                email_verified: data.email_verified,
                is_admin: data.is_admin,
                phone: data.phone,
                phone_verified: data.phone_verified,
                username: data.username,
            }),
            Err(_) => Err(String::from("unable to update the user")),
        }
    }
}

impl Crud for UserRepo {
    type ReadModel = User;
    type CreateModel = UserCreate;
    type UpdateModel = UserUpdate;

    async fn get_by_id(id: uuid::Uuid) -> Result<Self::ReadModel, String> {
        let db = pool().await.clone();
        let res = sqlx::query!("SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&db)
            .await;
        match res {
            Ok(data) => Ok(User {
                id: data.id,
                created_at: data.created_at,
                updated_at: data.updated_at,
                username: data.username,
                email: data.email,
                phone: data.phone,
                email_verified: data.email_verified,
                phone_verified: data.phone_verified,
                is_admin: data.is_admin,
            }),
            Err(_) => Err(String::from("cannot perform query")),
        }
    }

    async fn create(data: Self::CreateModel) -> Result<Self::ReadModel, String> {
        let db = pool().await.clone();
        let res = sqlx::query!(
            "
            INSERT INTO users (
                created_at,
                updated_at,
                password,
                username,
                phone,
                email,
                phone_verified,
                email_verified,
                is_admin
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, created_at, updated_at
        ",
            Utc::now(),
            Utc::now(),
            data.password,
            data.username,
            data.phone,
            data.email,
            data.phone_verified,
            data.email_verified,
            data.is_admin
        )
        .fetch_one(&db)
        .await;

        match res {
            Ok(rec) => Ok(User {
                id: rec.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                email: data.email,
                email_verified: data.email_verified,
                is_admin: data.is_admin,
                phone: data.phone,
                phone_verified: data.phone_verified,
                username: data.username,
            }),
            Err(_) => Err(String::from("cannot create the user")),
        }
    }

    async fn update(data: Self::UpdateModel) -> Result<Self::ReadModel, String> {
        let db = pool().await.clone();
        let res = sqlx::query!(
            "UPDATE users SET
            updated_at = $1,
            password = $2,
            username = $3,
            phone = $4,
            email = $5,
            phone_verified = $6,
            email_verified = $7,
            is_admin = $8
            WHERE id = $9 RETURNING created_at, updated_at",
            Utc::now(),
            data.password,
            data.username,
            data.phone,
            data.email,
            data.phone_verified,
            data.email_verified,
            data.is_admin,
            data.id,
        )
        .fetch_one(&db)
        .await;
        match res {
            Ok(rec) => Ok(User {
                id: data.id,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                email: data.email,
                email_verified: data.email_verified,
                is_admin: data.is_admin,
                phone: data.phone,
                phone_verified: data.phone_verified,
                username: data.username,
            }),
            Err(_) => Err(String::from("unable to update the user")),
        }
    }

    async fn delete(id: uuid::Uuid) -> Result<uuid::Uuid, String> {
        let db = pool().await.clone();
        let res = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&db)
            .await;
        match res {
            Ok(_) => Ok(id),
            Err(_) => Err(String::from("unable to delete the user")),
        }
    }
}
