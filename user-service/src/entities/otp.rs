use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Copy, Debug, PartialEq)]
pub enum OtpType {
    Unknown = 0,
    Verfiy = 1,
    Login = 2,
    ChangePassword = 3,
}

impl Clone for OtpType {
    fn clone(&self) -> Self {
        match self {
            Self::Unknown => Self::Unknown,
            Self::Verfiy => Self::Verfiy,
            Self::Login => Self::Login,
            Self::ChangePassword => Self::ChangePassword,
        }
    }
}

impl OtpType {
    pub fn from_int(n: i32) -> Self {
        match n {
            1 => OtpType::Verfiy,
            2 => OtpType::Login,
            3 => OtpType::ChangePassword,
            _ => OtpType::Unknown,
        }
    }

    pub fn to_int(self: Self) -> i32 {
        match self {
            OtpType::Unknown => 0,
            OtpType::Verfiy => 1,
            OtpType::Login => 2,
            OtpType::ChangePassword => 3,
        }
    }
}

#[derive(Copy, Debug, PartialEq)]
pub enum OtpMedia {
    Unknown = 0,
    Phone = 1,
    Email = 2,
}

impl Clone for OtpMedia {
    fn clone(&self) -> Self {
        match self {
            Self::Unknown => Self::Unknown,
            Self::Email => Self::Email,
            Self::Phone => Self::Phone,
        }
    }
}

impl OtpMedia {
    pub fn from_int(n: i32) -> Self {
        match n {
            1 => OtpMedia::Phone,
            2 => OtpMedia::Email,
            _ => OtpMedia::Unknown,
        }
    }

    pub fn to_int(self: Self) -> i32 {
        match self {
            OtpMedia::Unknown => 0,
            OtpMedia::Phone => 1,
            OtpMedia::Email => 2,
        }
    }
}

#[derive(Debug)]
pub struct Otp {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub key: String,
    pub is_used: bool,
    pub user_id: Uuid,
    pub r#type: OtpType,
    pub media: OtpMedia,
}

#[derive(Debug)]
pub struct OtpCreate {
    pub key: String,
    pub is_used: bool,
    pub user_id: Uuid,
    pub r#type: OtpType,
    pub media: OtpMedia,
}

#[derive(Debug)]
pub struct OtpUpdate {
    pub id: Uuid,
    pub key: String,
    pub is_used: bool,
    pub user_id: Uuid,
    pub r#type: OtpType,
    pub media: OtpMedia,
}
