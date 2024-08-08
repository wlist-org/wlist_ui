use crate::api::common::data::files::tokens::{FDownloadToken, FRefreshToken, FUploadToken};
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::confirmations::RefreshConfirmation)]
pub struct FRefreshConfirmation {
    pub files: Option<u64>,
    pub directories: Option<u64>,
    #[map(o2o::map(~))]
    pub token: FRefreshToken,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::confirmations::DownloadConfirmation)]
pub struct FDownloadConfirmation {
    pub size: u64,
    #[map(o2o::map(~))]
    pub token: FDownloadToken,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::confirmations::UploadConfirmation)]
pub struct FUploadConfirmation {
    pub done: bool,
    #[map(o2o::map(~))]
    pub token: FUploadToken,
}
