use std::fmt::Display;

use super::common::custom_local_date_format;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
        };
        write!(f, "{}", role)
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    username: String,
    password: String,
    #[serde(with = "custom_local_date_format")]
    create_at: DateTime<Local>,
    #[serde(with = "custom_local_date_format")]
    update_at: DateTime<Local>,
    pub role: UserRole,
}
