use crate::api::common::data::files::confirmations::FRefreshConfirmation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::files::progresses::FRefreshProgress;
use crate::api::common::data::files::tokens::FRefreshToken;
use crate::api::core::client::define_func;

define_func!(refresh_request(directory: FFileLocation) -> FRefreshConfirmation = wlist_native::core::client::refresh::refresh_request);
define_func!(refresh_cancel(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_cancel);
define_func!(refresh_confirm(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_confirm);
define_func!(refresh_pause(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_pause);
define_func!(refresh_resume(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_resume);
define_func!(refresh_is_paused(token: FRefreshToken) -> bool = wlist_native::core::client::refresh::refresh_is_paused);
define_func!(refresh_progress(token: FRefreshToken) -> FRefreshProgress = wlist_native::core::client::refresh::refresh_progress);
define_func!(refresh_check(token: FRefreshToken) -> bool = wlist_native::core::client::refresh::refresh_check);
