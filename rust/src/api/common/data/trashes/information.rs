use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::information::TrashInformation)]
pub struct FTrashInformation {
    pub id: i64,
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub name: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    pub trash_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::information::TrashDetailsInformation)]
pub struct FTrashDetailsInformation {
    #[map(o2o::map(~))]
    pub basic: FTrashInformation,
    #[from(o2o::from_option_arc(~))]
    #[into(o2o::into_option_arc(~))]
    pub md5: Option<String>,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::information::TrashListInformation)]
pub struct FTrashListInformation {
    pub total: u64,
    pub filtered: u64,
    #[map(o2o::map_vec(~))]
    pub files: Vec<FTrashInformation>,
}
