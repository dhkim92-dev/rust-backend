use shaku::module;
use crate::application::member::adapter::{MemberCreateUseCaseImpl, MemberDeleteUseCaseImpl, MemberUpdateUseCaseImpl};

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
            JwtServiceImpl,

            /* Member Service Related */
            MemberCreateUseCaseImpl,
            MemberDeleteUseCaseImpl,
            MemberUpdateUseCaseImpl,
            /* Board Service Related */
        ],
        providers = [
        ],
    }
}

