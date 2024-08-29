use wlist_native::common::data::files::confirmations::UploadConfirmation;
use wlist_native::common::data::files::FileLocation;
use wlist_native::common::data::files::information::FileInformation;
use wlist_native::common::data::files::options::Duplicate;
use wlist_native::common::data::files::tokens::UploadToken;

use crate::api::common::exceptions::UniverseError;
use crate::api::core::client::{define_func, PauseController, WlistClientManager};

define_func!(upload_check_name(name: String, parent: FileLocation, is_directory: bool) -> () = wlist_native::core::client::upload::upload_check_name);
define_func!(upload_mkdir(parent: FileLocation, name: String, duplicate: Duplicate) -> FileInformation = wlist_native::core::client::upload::upload_mkdir);
define_func!(upload_request(parent: FileLocation, name: String, size: u64, md5: String, md5s: Vec<String>, duplicate: Duplicate) -> UploadConfirmation = wlist_native::core::client::upload::upload_request);
define_func!(upload_cancel(token: UploadToken) -> () = wlist_native::core::client::upload::upload_cancel);
define_func!(upload_confirm(token: UploadToken) -> () = wlist_native::core::client::upload::upload_confirm);
define_func!(upload_finish(token: UploadToken) -> () = wlist_native::core::client::upload::upload_finish);

/// flutter_rust_bridge:ignore
mod internal {
    use bytes::Bytes;
    use tokio::sync::watch::Receiver;
    use super::*;

    define_func!(upload_stream(token: UploadToken, id: u64, buffer: &mut Bytes, control: Receiver<bool>) -> () = wlist_native::core::client::upload::upload_stream);
}

pub async fn upload_stream(client: Option<WlistClientManager>, token: UploadToken, id: u64, buffer: *const u8, buffer_size: usize, control: PauseController) -> Result<(), UniverseError> {
    let mut buffer = unsafe { wlist_native::core::helper::buffer::new_read_buffer(buffer, buffer_size) };
    internal::upload_stream(client, token, id, &mut buffer, control.sender.subscribe()).await
}
