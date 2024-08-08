use crate::api::common::data::storages::information::{FStorageDetailsInformation, FStorageInformation, FStorageListInformation};
use crate::api::common::data::storages::options::FListStorageOptions;
use crate::api::common::o2o;
use crate::api::core::client::define_func;

define_func!(storages_list(options: FListStorageOptions) -> FStorageListInformation = wlist_native::core::client::storages::storages_list);
define_func!(storages_get(id: i64, check: bool) -> FStorageDetailsInformation = wlist_native::core::client::storages::storages_get);
define_func!(storages_remove(id: i64) -> () = wlist_native::core::client::storages::storages_remove);
define_func!(storages_set_readonly(id: i64, readonly: bool) -> () = wlist_native::core::client::storages::storages_set_readonly);

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::core::client::storages::LanzouConfigurationInner)]
pub struct FLanzouConfiguration {
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub phone_number: String,
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub password: String,
    pub root_directory_id: i64,
}

define_func!(storages_lanzou_add(name: String, config: FLanzouConfiguration) -> FStorageInformation = wlist_native::core::client::storages::storages_lanzou_add);
define_func!(storages_lanzou_update(id: i64, config: FLanzouConfiguration) -> () = wlist_native::core::client::storages::storages_lanzou_update);
define_func!(storages_lanzou_chcek(config: FLanzouConfiguration) -> () = wlist_native::core::client::storages::storages_lanzou_chcek);
