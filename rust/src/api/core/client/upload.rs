use std::fs::File;

use wlist_native::common::data::files::confirmations::UploadConfirmation;
use wlist_native::common::data::files::FileLocation;
use wlist_native::common::data::files::information::FileInformation;
use wlist_native::common::data::files::options::Duplicate;
use wlist_native::common::data::files::tokens::UploadToken;

use crate::api::common::data::files::information::{FFileInformation, FUploadInformation};
use crate::api::common::exceptions::UniverseError;
use crate::api::core::client::{define_func, PauseController, WlistClientManager};

define_func!(
    /// Check whether the file/directory name is valid.
    ///
    /// Notice that this method only provides fast filtering, and some cases may still not be covered.
    ///
    /// parent: .isDirectory == true
    ///
    /// May returns [NameTooLongError], [InvalidFilenameError], [IllegalSuffixError] and [DuplicateFileError].
    /// The [DuplicateFileError] is the last error to check.
    ///
    /// >[NameTooLongError]: crate::api::common::exceptions::UniverseError::NameTooLongError
    /// >[InvalidFilenameError]: crate::api::common::exceptions::UniverseError::InvalidFilenameError
    /// >[IllegalSuffixError]: crate::api::common::exceptions::UniverseError::IllegalSuffixError
    /// >[DuplicateFileError]: crate::api::common::exceptions::UniverseError::DuplicateFileError
    upload_check_name(name: String, parent: FileLocation, is_directory: bool) -> () = wlist_native::core::client::upload::upload_check_name
);
define_func!(
    /// Create a new empty directory.
    ///
    ///
    /// parent: .isDirectory == true
    ///
    /// name: 0 < len < 32768
    upload_mkdir(parent: FileLocation, name: String, duplicate: Duplicate) -> FileInformation = wlist_native::core::client::upload::upload_mkdir
);

define_func!(
    /// Request to upload a new file.
    ///
    ///
    /// parent: .isDirectory == true
    ///
    /// name: 0 < len < 32768
    ///
    /// md5: the hash md5 of the entire new file. (This should be a lowercase string with a length of 32.)
    ///
    /// md5s: the md5 slice of each 4MB part of the new file.
    upload_request(parent: FileLocation, name: String, size: u64, md5: String, md5s: Vec<String>, duplicate: Duplicate) -> UploadConfirmation = wlist_native::core::client::upload::upload_request
);
define_func!(
    /// Cancel an upload.
    ///
    /// What ever the upload is paused or not, or not confirmed, it will be canceled.
    upload_cancel(token: UploadToken) -> () = wlist_native::core::client::upload::upload_cancel
);
define_func!(
    /// Confirm an upload.
    ///
    /// Then the upload is automatically resumed.
    upload_confirm(token: UploadToken) -> FUploadInformation = wlist_native::core::client::upload::upload_confirm
);
define_func!(
    /// Finish an upload.
    ///
    /// May return [UploadChunkIncompleteError](crate::api::common::exceptions::UniverseError::UploadChunkIncompleteError).
    upload_finish(token: UploadToken) -> FFileInformation = wlist_native::core::client::upload::upload_finish
);

/// flutter_rust_bridge:ignore
mod internal {
    use bytes::Bytes;
    use tokio::sync::watch::Receiver;
    use super::*;

    define_func!(upload_stream(token: UploadToken, id: u64, buffer: &mut Bytes, control: Receiver<bool>) -> () = wlist_native::core::client::upload::upload_stream);
}

/// Upload the file chunk.
///
///
/// id: see the `chunks` field in [FUploadInformation]. (0 <= id < chunks_length)
///
/// buffer: a pointer to the buffer to read the data.
pub async fn upload_stream(client: Option<WlistClientManager>, token: UploadToken, id: u64, buffer: *const u8, buffer_size: usize, control: PauseController) -> Result<(), UniverseError> {
    let mut buffer = unsafe { wlist_native::core::helper::buffer::new_read_buffer(buffer, buffer_size) };
    internal::upload_stream(client, token, id, &mut buffer, control.sender.subscribe()).await
}
