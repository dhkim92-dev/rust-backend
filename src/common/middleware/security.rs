pub struct LoginMember {
    pub id: uuid::Uuid,
    pub nickname: String,
    pub email: String,
    pub role: String,
    pub is_activated: bool,
}

