use crate::application::member::adapter::{
    MemberCreateUseCaseImpl, MemberDeleteUseCaseImpl, MemberUpdateUseCaseImpl,
};
use shaku::module;

use super::application::auth::auth_service::AuthService;
use super::common::database::DbConnProviderImpl;
use super::common::jwt::JwtServiceImpl;
use super::config::ConfigProviderImpl;
use super::domain::member::repository::{MemberCommandRepository, MemberQueryRepository};

module! {
    pub AppContext {
        components = [
            // basement
            DbConnProviderImpl,
            ConfigProviderImpl,

            //
            AuthService,
            MemberQueryRepository,
            MemberCommandRepository,

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
