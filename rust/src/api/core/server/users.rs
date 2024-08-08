pub async fn reset_admin_password() -> anyhow::Result<String> {
    wlist_native::core::server::users::reset_admin_password().await
}
