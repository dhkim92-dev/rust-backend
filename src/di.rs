use shaku::module;
use super::application::auth::auth_service::AuthService;
use super::domain::member::repository::MemberQueryRepository;
use super::config::ConfigProviderImpl;
use super::common::jwt::JwtServiceImpl;

module! {
    pub AppContext {
        components = [
            AuthService, 
            MemberQueryRepository,
            ConfigProviderImpl,
            JwtServiceImpl
        ],
        providers = [
        ],
    }
}

