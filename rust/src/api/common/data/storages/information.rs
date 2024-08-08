use crate::api::common::data::storages::FStorageType;
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageInformation)]
pub struct FStorageInformation {
    pub id: i64,
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub name: String,
    pub read_only: bool,
    #[map(o2o::map(~))]
    pub storage_type: FStorageType,
    pub available: bool,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub update_time: chrono::DateTime<chrono::Utc>,
    pub root_directory_id: i64,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageDetailsInformation)]
pub struct FStorageDetailsInformation {
    #[map(o2o::map(~))]
    pub basic: FStorageInformation,
    pub size: Option<u64>,
    pub indexed_size: u64,
    pub total_size: Option<u64>,
    #[map(o2o::map_option(~))]
    pub upload_flow: Option<FStorageFlow>,
    #[map(o2o::map_option(~))]
    pub download_flow: Option<FStorageFlow>,
    pub max_size_per_file: u64,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageListInformation)]
pub struct FStorageListInformation {
    pub total: u64,
    pub filtered: u64,
    #[map(o2o::map_vec(~))]
    pub storages: Vec<FStorageInformation>,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageFlow)]
pub struct FStorageFlow {
    pub start: chrono::DateTime<chrono::Utc>,
    pub refresh: chrono::DateTime<chrono::Utc>,
    pub flow: u64,
}
