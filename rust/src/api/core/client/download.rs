use wlist_native::common::data::files::confirmations::DownloadConfirmation;
use wlist_native::common::data::files::FileLocation;
use wlist_native::common::data::files::tokens::DownloadToken;

use crate::api::common::exceptions::UniverseError;
use crate::api::core::client::{define_func, PauseController, WlistClientManager};

define_func!(download_request(file: FileLocation, from: u64, to: u64) -> DownloadConfirmation = wlist_native::core::client::download::download_request);
define_func!(download_cancel(token: DownloadToken) -> () = wlist_native::core::client::download::download_cancel);
define_func!(download_confirm(token: DownloadToken) -> () = wlist_native::core::client::download::download_confirm);
define_func!(download_finish(token: DownloadToken) -> () = wlist_native::core::client::download::download_finish);

/// flutter_rust_bridge:ignore
mod internal {
    use bytes::BufMut;
    use tokio::sync::broadcast::Receiver;
    use super::*;

    define_func!(download_stream(token: DownloadToken, id: u64, start: u64, buffer: &mut impl BufMut, control: Receiver<bool>) -> () = wlist_native::core::client::download::download_stream);
}

pub async fn download_stream(client: Option<WlistClientManager>, token: DownloadToken, id: u64, start: u64, buffer: *mut u8, buffer_size: usize, control: PauseController) -> Result<(), UniverseError> {
    let mut buffer = unsafe { wlist_native::core::helper::buffer::WriteBuffer::new(buffer, buffer_size) };
    internal::download_stream(client, token, id, start, &mut buffer, control.sender.subscribe()).await
}
