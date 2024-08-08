use crate::api::common::o2o;
use crate::api::common::data::files::confirmations::FDownloadConfirmation;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::FileInformation)]
pub struct FFileInformation {
    pub id: i64,
    pub parent_id: i64,
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub name: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[flutter_rust_bridge::frb(mirror(FileDetailsInformation), non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::FileDetailsInformation)]
pub struct FFileDetailsInformation {
    #[map(o2o::map(~))]
    pub basic: FFileInformation,
    #[from(o2o::from_option_arc(~))]
    #[into(o2o::into_option_arc(~))]
    pub md5: Option<String>,
    pub path: Vec<String>,
    #[map(o2o::map_option(~))]
    pub thumbnail: Option<FDownloadConfirmation>,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::FileListInformation)]
pub struct FFileListInformation {
    pub total: u64,
    pub filtered: u64,
    #[map(o2o::map_vec(~))]
    pub files: Vec<FFileInformation>,
}


#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::DownloadChunkInformation)]
pub struct FDownloadChunkInformation {
    pub range: bool,
    pub start: u64,
    pub size: u64,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::DownloadInformation)]
pub struct FDownloadInformation {
    #[map(o2o::map_vec(~))]
    pub chunks: Vec<FDownloadChunkInformation>,
    pub expire: chrono::DateTime<chrono::Utc>,
}


#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::UploadChunkInformation)]
pub struct FUploadChunkInformation {
    pub start: u64,
    pub size: u64,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::UploadInformation)]
pub struct FUploadInformation {
    #[map(o2o::map_vec(~))]
    pub chunks: Vec<FUploadChunkInformation>,
    pub expire: chrono::DateTime<chrono::Utc>,
}


#[flutter_rust_bridge::frb(mirror(ShareInformation), non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::ShareInformation)]
pub struct FShareInformation {
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub id: String,
    #[from(o2o::from_option_arc(~))]
    #[into(o2o::into_option_arc(~))]
    pub password: Option<String>,
    pub expire: chrono::DateTime<chrono::Utc>,
}
