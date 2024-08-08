use crate::api::core::client::define_func;

define_func!(users_login(username: String, password: String) -> () = wlist_native::core::client::users::users_login);
define_func!(users_logout() -> () = wlist_native::core::client::users::users_logout);
