pub mod information;
pub mod options;

#[flutter_rust_bridge::frb(non_opaque)]
/// All supported storage types.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::StorageType)]
pub enum FStorageType {
    /// [lanzou](https://up.woozooo.com/).
    Lanzou,

}
