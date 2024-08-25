pub mod account {
    use crate::api::common::exceptions::UniverseError;

    pub async fn login(user_id: String, password: String) -> Result<(), UniverseError> {
        wlist_native::web::account::login::login(user_id, password).await.map_err(Into::into)
    }

    pub async fn logout() -> Result<(), UniverseError> {
        wlist_native::web::account::logout::logout().await.map_err(Into::into)
    }
}

pub mod register {
    use crate::api::common::exceptions::UniverseError;

    pub async fn register_as_guest(device_id: String, password: String) -> Result<Option<String>, UniverseError> {
        wlist_native::web::register::as_guest::register_as_guest(device_id, password).await.map_err(Into::into)
    }

    pub async fn unregister(passport: String) -> Result<(), UniverseError> {
        wlist_native::web::register::unregister::unregister(passport).await.map_err(Into::into)
    }
}

pub mod user {
    use crate::api::common::exceptions::UniverseError;

    pub async fn get_nickname() -> Result<String, UniverseError> {
        wlist_native::web::user::nickname::get_nickname().await.map_err(Into::into)
    }

    pub async fn set_nickname(nickname: String) -> Result<(), UniverseError> {
        wlist_native::web::user::nickname::set_nickname(nickname).await.map_err(Into::into)
    }

    pub async fn reset_password(old: String, new: String) -> Result<(), UniverseError> {
        wlist_native::web::user::password::reset_password(old, new).await.map_err(Into::into)
    }
}

pub mod version {
    use crate::api::common::exceptions::UniverseError;

    #[flutter_rust_bridge::frb(non_opaque)]
    #[derive(o2o::o2o)]
    #[map_owned(wlist_native::web::version::VersionState)]
    pub enum FVersionState {
        Latest,
        Updatable,
        Unavailable,
    }

    pub async fn check_version() -> Result<FVersionState, UniverseError> {
        wlist_native::web::version::check_version().await.map(Into::into).map_err(Into::into)
    }
}
