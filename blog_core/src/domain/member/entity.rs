use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct MemberEntity {
    pub id: Option<Uuid>,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_activated: bool,
}
