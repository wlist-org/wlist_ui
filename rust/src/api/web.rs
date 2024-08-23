pub mod account {
    pub async fn login(user_id: String, password: String) -> anyhow::Result<()> {
        wlist_native::web::account::login::login(user_id, password).await
    }

    pub async fn logout() -> anyhow::Result<()> {
        wlist_native::web::account::logout::logout().await
    }
}

pub mod register {
    pub async fn register_as_guest(device_id: String, passport: String) -> anyhow::Result<Option<String>> {
        wlist_native::web::register::as_guest::register_as_guest(device_id, passport).await
    }

    pub async fn unregister(passport: String) -> anyhow::Result<()> {
        wlist_native::web::register::unregister::unregister(passport).await
    }
}

pub mod user {
    pub async fn get_nickname() -> anyhow::Result<String> {
        wlist_native::web::user::nickname::get_nickname().await
    }

    pub async fn set_nickname(nickname: String) -> anyhow::Result<()> {
        wlist_native::web::user::nickname::set_nickname(nickname).await
    }

    pub async fn reset_password(old: String, new: String) -> anyhow::Result<()> {
        wlist_native::web::user::password::reset_password(old, new).await
    }
}

pub mod version {
    #[flutter_rust_bridge::frb(non_opaque)]
    #[derive(o2o::o2o)]
    #[map_owned(wlist_native::web::version::VersionState)]
    pub enum FVersionState {
        Latest,
        Updatable,
        Unavailable,
    }

    pub async fn check_version() -> anyhow::Result<FVersionState> {
        wlist_native::web::version::check_version().await.map(Into::into)
    }
}
