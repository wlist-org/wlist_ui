use crate::api::common::exceptions::UniverseError;

pub async fn reset_admin_password() -> Result<String, UniverseError> {
    wlist_native::core::server::users::reset_admin_password().await.map_err(Into::into)
}
