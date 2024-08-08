use crate::api::common::data::FDirection;
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::FilesFilter)]
pub enum FFilesFilter {
    OnlyDirectories,
    OnlyFiles,
    Both,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::FilesOrder)]
pub enum FFilesOrder {
    Id,
    Name,
    Suffix,
    Directory,
    Size,
    CreateTime,
    UpdateTime,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::ListFileOptions)]
pub struct FListFileOptions {
    #[map(o2o::map(~))]
    pub filter: FFilesFilter,
    #[from(o2o::from_index_map(~))]
    #[into(o2o::into_index_map(~))]
    pub orders: Vec<(FFilesOrder, FDirection)>,
    pub offset: u64,
    pub limit: u32,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::Duplicate)]
pub enum FDuplicate {
    Error,
    Replace,
    Rename,
    RenameIfDifferent,
}


#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::SearchPattern)]
pub enum FSearchPattern {
    FullMatch,
    Regex,
    Pinyin,
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::SearchFilesOptions)]
pub struct FSearchFilesOptions {
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub keyword: String,
    #[map(o2o::map(~))]
    pub pattern: FSearchPattern,
    pub recursive: bool,
}
