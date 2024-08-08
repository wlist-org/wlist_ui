#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::progresses::RefreshProgress)]
pub struct FRefreshProgress {
    pub loaded_files: u64,
    pub loaded_directories: u64,
    pub total_files: u64,
    pub total_directories: u64,
}
