use std::fs::File;

use wlist_native::common::data::files::confirmations::DownloadConfirmation;
use wlist_native::common::data::files::FileLocation;
use wlist_native::common::data::files::tokens::DownloadToken;

use crate::api::common::data::files::information::FDownloadInformation;
use crate::api::common::exceptions::UniverseError;
use crate::api::core::client::{define_func, PauseController, WlistClientManager};

define_func!(
    /// Request to download the file.
    ///
    /// Notice that the upload token will lock the file until it is canceled/finished.
    ///
    ///
    /// file: .isDirectory == false
    ///
    /// from: the start byte index of the entire file. (include) (0 <= from <= to)
    ///
    /// to: the last byte index of the entire file. (include) (For entire file, you can pass a value large enough.)
    download_request(file: FileLocation, from: u64, to: u64) -> DownloadConfirmation = wlist_native::core::client::download::download_request
);
define_func!(
    /// Cancel a download.
    ///
    /// What ever the download is paused or not, or not confirmed, it will be canceled.
    download_cancel(token: DownloadToken) -> () = wlist_native::core::client::download::download_cancel
);
define_func!(
    /// Confirm a download.
    ///
    /// Then the download is automatically resumed.
    download_confirm(token: DownloadToken) -> FDownloadInformation = wlist_native::core::client::download::download_confirm
);
define_func!(
    /// Finish a download.
    ///
    /// This function is similar to call [download_cancel], but marks the download as finished.
    download_finish(token: DownloadToken) -> () = wlist_native::core::client::download::download_finish
);

/// flutter_rust_bridge:ignore
mod internal {
    use bytes::BufMut;
    use tokio::sync::watch::Receiver;
    use super::*;

    define_func!(download_stream(token: DownloadToken, id: u64, start: u64, buffer: &mut impl BufMut, control: Receiver<bool>) -> () = wlist_native::core::client::download::download_stream);
}

/// Download the file chunk.
///
///
/// id: see the `chunks` field in [FDownloadInformation]. (0 <= id < chunks_length)
///
/// start: the start position to download of this chunk. (0 <= start <= chunk_size)
///
/// buffer: a pointer to the buffer to write the data.
pub async fn download_stream(client: Option<WlistClientManager>, token: DownloadToken, id: u64, start: u64, buffer: *mut u8, buffer_size: usize, control: PauseController) -> Result<(), UniverseError> {
    let mut buffer = unsafe { wlist_native::core::helper::buffer::WriteBuffer::new(buffer, buffer_size) };
    internal::download_stream(client, token, id, start, &mut buffer, control.sender.subscribe()).await
}
