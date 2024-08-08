use crate::api::common::data::FDirection;
use crate::api::common::data::files::options::{FFilesFilter, FSearchPattern};
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::options::TrashesOrder)]
pub enum FTrashesOrder {
    Id,
    Name,
    Suffix,
    Directory,
    Size,
    CreateTime,
    UpdateTime,
    TrashTime,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::options::ListTrashOptions)]
pub struct FListTrashOptions {
    #[map(o2o::map(~))]
    pub filter: FFilesFilter,
    #[from(o2o::from_index_map(~))]
    #[into(o2o::into_index_map(~))]
    pub orders: Vec<(FTrashesOrder, FDirection)>,
    pub offset: u64,
    pub limit: u32,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::options::SearchTrashesOptions)]
pub struct FSearchTrashesOptions {
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub keyword: String,
    #[map(o2o::map(~))]
    pub pattern: FSearchPattern,
}
