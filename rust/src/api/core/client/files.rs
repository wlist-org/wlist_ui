use crate::api::common::data::FEither;
use crate::api::common::data::files::confirmations::FRefreshConfirmation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::files::information::{FFileDetailsInformation, FFileInformation, FFileListInformation};
use crate::api::common::data::files::options::{FDuplicate, FListFileOptions};
use crate::api::core::client::define_func;

define_func!(files_list(directory: FFileLocation, options: FListFileOptions) -> FEither<FFileListInformation, FRefreshConfirmation> = wlist_native::core::client::files::files_list);
define_func!(files_get(location: FFileLocation, check: bool) -> FFileDetailsInformation = wlist_native::core::client::files::files_get);

define_func!(files_copy(source: FFileLocation, target: FFileLocation, name: String, duplicate: FDuplicate) -> FFileInformation = wlist_native::core::client::files::files_copy);
define_func!(files_move(source: FFileLocation, target: FFileLocation, duplicate: FDuplicate) -> FFileInformation = wlist_native::core::client::files::files_move);
define_func!(files_rename(location: FFileLocation, name: String, duplicate: FDuplicate) -> FFileInformation = wlist_native::core::client::files::files_rename);
