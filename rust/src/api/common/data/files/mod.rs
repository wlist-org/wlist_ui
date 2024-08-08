pub mod tokens;
pub mod confirmations;
pub mod information;
pub mod progresses;
pub mod options;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::FileLocation)]
pub struct FFileLocation {
    pub storage: i64,
    pub file_id: i64,
    pub is_directory: bool,
}
