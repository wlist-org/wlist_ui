use crate::api::common::data::FEither;
use crate::api::common::data::files::confirmations::FRefreshConfirmation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::files::information::FFileInformation;
use crate::api::common::data::trashes::information::{FTrashDetailsInformation, FTrashInformation, FTrashListInformation};
use crate::api::common::data::trashes::options::FListTrashOptions;
use crate::api::core::client::define_func;

define_func!(trash_list(storage: i64, options: FListTrashOptions) -> FEither<FTrashListInformation, FRefreshConfirmation> = wlist_native::core::client::trash::trash_list);
define_func!(trash_refresh(storage: i64) -> FRefreshConfirmation = wlist_native::core::client::trash::trash_refresh);
define_func!(trash_get(location: FFileLocation, check: bool) -> FTrashDetailsInformation = wlist_native::core::client::trash::trash_get);
define_func!(trash_trash(location: FFileLocation) -> FTrashInformation = wlist_native::core::client::trash::trash_trash);
define_func!(trash_restore(location: FFileLocation, parent: i64) -> FFileInformation = wlist_native::core::client::trash::trash_restore);
define_func!(trash_delete(location: FFileLocation) -> () = wlist_native::core::client::trash::trash_delete);
define_func!(trash_delete_all(storage: i64) -> () = wlist_native::core::client::trash::trash_delete_all);
