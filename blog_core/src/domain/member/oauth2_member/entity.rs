use uuid::Uuid;


pub struct OAuth2MemberEntity {
    id: Option<Uuid>,
    member_id: Uuid,
    provider: String,
    user_id: String,
    email: Option<String>,
    access_token: String
}

impl OAuth2MemberEntity {
    pub fn new(
        id: Option<Uuid>,
        member_id: Uuid,
        provider: String,
        user_id: String,
        email: Option<String>,
        access_token: String
    ) -> Self {
        Self {
            id,
            member_id,
            provider,
            user_id,
            email,
            access_token
        }
    }

    pub fn get_id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn get_member_id(&self) -> Uuid {
        self.member_id
    }

    pub fn get_provider(&self) -> &String {
        &self.provider
    }

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_email(&self) -> &Option<String> {
        &self.email
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
}
