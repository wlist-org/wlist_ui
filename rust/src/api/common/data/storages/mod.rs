pub mod information;
pub mod options;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::StorageType)]
pub enum FStorageType {
    Lanzou,

}
