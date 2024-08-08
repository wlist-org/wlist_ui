use crate::api::common::data::FDirection;
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::options::StoragesFilter)]
pub enum FStoragesFilter {
    Readonly,
    Writable,
    Shared,
    Private,
    ReadonlyPrivate,
    Owned,
    All,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::options::StoragesOrder)]
pub enum FStoragesOrder {
    Id,
    Name,
    Shared,
    Readonly,
    Size,
    IndexedSize,
    TotalSize,
    SpareSize,
    CreateTime,
    UpdateTime,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::options::ListStorageOptions)]
pub struct FListStorageOptions {
    #[map(o2o::map(~))]
    pub filter: FStoragesFilter,
    #[from(o2o::from_index_map(~))]
    #[into(o2o::into_index_map(~))]
    pub orders: Vec<(FStoragesOrder, FDirection)>,
    pub offset: u64,
    pub limit: u32,
}
