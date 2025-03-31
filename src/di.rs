use shaku::module;
use super::application::auth::auth_service::AuthService;
use super::domain::member::repository::MemberQueryRepository;

module! {
    pub AppContext {
        components = [
            AuthService, 
            MemberQueryRepository,
        ],
        providers = [
        ],
    }
}
