use crate::application::member::adapter::{
    MemberCreateUseCaseImpl, MemberDeleteUseCaseImpl, MemberUpdateUseCaseImpl,
};
use crate::common::CookieBuilderImpl;
use shaku::module;

use super::application::auth::*;
use super::common::database::DbConnProviderImpl;
use super::common::jwt::JwtServiceImpl;
use super::config::ConfigProviderImpl;
use super::domain::member::repository::{MemberCommandRepository, MemberQueryRepository};
use super::domain::board::repository::{SeaOrmLoadBoardAdapter, SeaOrmSaveBoardAdapter};

module! {
    pub AppContext {
        components = [
            // basement
            DbConnProviderImpl,
            ConfigProviderImpl,
            CookieBuilderImpl,

            // Auth
            AuthService,
            JwtUseCaseImpl,
            JwtServiceImpl,

            // Member
            MemberQueryRepository,
            MemberCommandRepository,

            // Board,
            SeaOrmLoadBoardAdapter,
            SeaOrmSaveBoardAdapter,

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
