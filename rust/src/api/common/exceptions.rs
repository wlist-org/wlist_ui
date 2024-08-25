use std::borrow::Cow;
use std::sync::Arc;

use hashbrown::HashMap;
use wlist_native::common::data::files::FileLocation;

macro_rules! define_exception {
    ($frb: ident $exception: ident $origin: ident() $msg: literal) => {
        pub use ::wlist_native::common::exceptions::$origin;
        #[::flutter_rust_bridge::frb(mirror($origin), opaque)]
        pub struct $exception {}
        define_exception!(fn $frb $origin);
    };
    ($frb: ident $exception: ident $origin: ident($($field: ident: $ty: ty),+) |&$self: ident, $f: ident| $write: expr) => {
        pub use ::wlist_native::common::exceptions::$origin;
        #[allow(dead_code)]
        #[::flutter_rust_bridge::frb(mirror($origin), opaque)]
        pub struct $exception {
            $($field: $ty),+
        }
        define_exception!(fn $frb $origin);
    };
    (fn $frb: ident $origin: ident) => {
        #[allow(dead_code, non_snake_case)]
        #[flutter_rust_bridge::frb(sync, getter)]
        pub fn $frb() -> $origin {
            unreachable!()
        }
    };
}

define_exception!(__UnavailableApiVersionError _UnavailableApiVersionError UnavailableApiVersionError() "current api version is unavailable");
define_exception!(__MatchFrequencyControlError _MatchFrequencyControlError MatchFrequencyControlError() "match frequency control");
define_exception!(__IncorrectArgumentError _IncorrectArgumentError IncorrectArgumentError(e: Cow<'static, str>) |&e, f| write!(f, "incorrect argument: {}", e.e));
define_exception!(__TooLargeDataError _TooLargeDataError TooLargeDataError() "too large data");
define_exception!(__PasswordMismatchedError _PasswordMismatchedError PasswordMismatchedError() "password is mismatched");
define_exception!(__TokenExpiredError _TokenExpiredError TokenExpiredError() "token is expired");

define_exception!(__InvalidStorageConfigError _InvalidStorageConfigError InvalidStorageConfigError(e: HashMap<String, String>) |&e, f| write!(f, "invalid storage config: {:?}", e.e));
define_exception!(__IncorrectStorageAccountError _IncorrectStorageAccountError IncorrectStorageAccountError() "incorrect storage account");
define_exception!(__DuplicateStorageError _DuplicateStorageError DuplicateStorageError() "duplicate storage");
define_exception!(__StorageNotFoundError _StorageNotFoundError StorageNotFoundError() "storage not found");
define_exception!(__StorageTypeMismatchedError _StorageTypeMismatchedError StorageTypeMismatchedError() "storage type is mismatched");
define_exception!(__StorageInLockError _StorageInLockError StorageInLockError(e: Cow<'static, str>) |&e, f| write!(f, "storage in lock: {}", e.e));

define_exception!(__ComplexOperationError _ComplexOperationError ComplexOperationError() "operation is too complex");
define_exception!(__DuplicateFileError _DuplicateFileError DuplicateFileError(l: FileLocation, name: Arc<String>) |&e, f| write!(f, "duplicate file {} in {:?}", e.name, e.l));
define_exception!(__FileNotFoundError _FileNotFoundError FileNotFoundError(l: FileLocation) |&e, f| write!(f, "file not found {:?}", e.l));
define_exception!(__FileInLockError _FileInLockError FileInLockError(l: FileLocation, e: Cow<'static, str>) |&e, f| write!(f, "{} (locking {:?})", e.e, e.l));
define_exception!(__UploadChunkIncompleteError _UploadChunkIncompleteError UploadChunkIncompleteError() "upload chunks are incomplete");

define_exception!(__IllegalSuffixError _IllegalSuffixError IllegalSuffixError(suffix: Cow<'static, str>) |&e, f| write!(f, "illegal suffix: {}", e.suffix));
define_exception!(__InvalidFilenameError _InvalidFilenameError InvalidFilenameError(name: Arc<String>, ch: Option<char>) |&e, f| write!(f, "invalid filename: {} char: {:?}", e.name, e.ch));
define_exception!(__NameTooLongError _NameTooLongError NameTooLongError(name: Arc<String>, max: Option<u64>) |&e, f| write!(f, "name too long: {} limit: {:?}", e.name, e.max));
define_exception!(__ReadOnlyStorageError _ReadOnlyStorageError ReadOnlyStorageError() "readonly storage");
define_exception!(__SpaceNotEnoughError _SpaceNotEnoughError SpaceNotEnoughError(left: Option<u64>, need: Option<u64>) |&e, f| write!(f, "space not enough: {:?} < {:?}", e.left, e.need));
define_exception!(__FlowNotEnoughError _FlowNotEnoughError FlowNotEnoughError(upload: bool, left: Option<u64>, need: Option<u64>) |&e, f| write!(f, "{} flow not enough: {:?} < {:?}", if e.upload { "upload" } else { "download" }, e.left, e.need));
define_exception!(__FileTooLargeError _FileTooLargeError FileTooLargeError(size: u64, max: Option<u64>) |&e, f| write!(f, "file too large: {} limit: {:?}", e.size, e.max));


#[flutter_rust_bridge::frb(non_opaque)]
#[derive(Debug)]
pub enum UniverseError {
    IO(Option<String>),
    Network(String),

    UnavailableApiVersionError(UnavailableApiVersionError),
    MatchFrequencyControlError(MatchFrequencyControlError),
    IncorrectArgumentError(IncorrectArgumentError),
    TooLargeDataError(TooLargeDataError),
    PasswordMismatchedError(PasswordMismatchedError),
    TokenExpiredError(TokenExpiredError),

    InvalidStorageConfigError(InvalidStorageConfigError),
    IncorrectStorageAccountError(IncorrectStorageAccountError),
    DuplicateStorageError(DuplicateStorageError),
    StorageNotFoundError(StorageNotFoundError),
    StorageTypeMismatchedError(StorageTypeMismatchedError),
    StorageInLockError(StorageInLockError),

    ComplexOperationError(ComplexOperationError),
    DuplicateFileError(DuplicateFileError),
    FileNotFoundError(FileNotFoundError),
    FileInLockError(FileInLockError),
    UploadChunkIncompleteError(UploadChunkIncompleteError),

    IllegalSuffixError(IllegalSuffixError),
    InvalidFilenameError(InvalidFilenameError),
    NameTooLongError(NameTooLongError),
    ReadOnlyStorageError(ReadOnlyStorageError),
    SpaceNotEnoughError(SpaceNotEnoughError),
    FlowNotEnoughError(FlowNotEnoughError),
    FileTooLargeError(FileTooLargeError),

    Other(String),
}

impl From<anyhow::Error> for UniverseError {
    fn from(value: anyhow::Error) -> Self {
        wlist_native::common::exceptions::UniverseError::new(value).into()
    }
}

impl From<wlist_native::common::exceptions::UniverseError> for UniverseError {
    fn from(value: wlist_native::common::exceptions::UniverseError) -> UniverseError {
        match value {
            wlist_native::common::exceptions::UniverseError::IO(_f0, f1, ) => UniverseError::IO(f1),
            wlist_native::common::exceptions::UniverseError::Network(f0, ) => UniverseError::Network(f0),
            wlist_native::common::exceptions::UniverseError::UnavailableApiVersionError(f0, ) => UniverseError::UnavailableApiVersionError(f0),
            wlist_native::common::exceptions::UniverseError::MatchFrequencyControlError(f0, ) => UniverseError::MatchFrequencyControlError(f0),
            wlist_native::common::exceptions::UniverseError::IncorrectArgumentError(f0, ) => UniverseError::IncorrectArgumentError(f0),
            wlist_native::common::exceptions::UniverseError::TooLargeDataError(f0, ) => UniverseError::TooLargeDataError(f0),
            wlist_native::common::exceptions::UniverseError::PasswordMismatchedError(f0, ) => UniverseError::PasswordMismatchedError(f0),
            wlist_native::common::exceptions::UniverseError::TokenExpiredError(f0, ) => UniverseError::TokenExpiredError(f0),
            wlist_native::common::exceptions::UniverseError::InvalidStorageConfigError(f0, ) => UniverseError::InvalidStorageConfigError(f0),
            wlist_native::common::exceptions::UniverseError::IncorrectStorageAccountError(f0, ) => UniverseError::IncorrectStorageAccountError(f0),
            wlist_native::common::exceptions::UniverseError::DuplicateStorageError(f0, ) => UniverseError::DuplicateStorageError(f0),
            wlist_native::common::exceptions::UniverseError::StorageNotFoundError(f0, ) => UniverseError::StorageNotFoundError(f0),
            wlist_native::common::exceptions::UniverseError::StorageTypeMismatchedError(f0, ) => UniverseError::StorageTypeMismatchedError(f0),
            wlist_native::common::exceptions::UniverseError::StorageInLockError(f0, ) => UniverseError::StorageInLockError(f0),
            wlist_native::common::exceptions::UniverseError::ComplexOperationError(f0, ) => UniverseError::ComplexOperationError(f0),
            wlist_native::common::exceptions::UniverseError::DuplicateFileError(f0, ) => UniverseError::DuplicateFileError(f0),
            wlist_native::common::exceptions::UniverseError::FileNotFoundError(f0, ) => UniverseError::FileNotFoundError(f0),
            wlist_native::common::exceptions::UniverseError::FileInLockError(f0, ) => UniverseError::FileInLockError(f0),
            wlist_native::common::exceptions::UniverseError::UploadChunkIncompleteError(f0, ) => UniverseError::UploadChunkIncompleteError(f0),
            wlist_native::common::exceptions::UniverseError::IllegalSuffixError(f0, ) => UniverseError::IllegalSuffixError(f0),
            wlist_native::common::exceptions::UniverseError::InvalidFilenameError(f0, ) => UniverseError::InvalidFilenameError(f0),
            wlist_native::common::exceptions::UniverseError::NameTooLongError(f0, ) => UniverseError::NameTooLongError(f0),
            wlist_native::common::exceptions::UniverseError::ReadOnlyStorageError(f0, ) => UniverseError::ReadOnlyStorageError(f0),
            wlist_native::common::exceptions::UniverseError::SpaceNotEnoughError(f0, ) => UniverseError::SpaceNotEnoughError(f0),
            wlist_native::common::exceptions::UniverseError::FlowNotEnoughError(f0, ) => UniverseError::FlowNotEnoughError(f0),
            wlist_native::common::exceptions::UniverseError::FileTooLargeError(f0, ) => UniverseError::FileTooLargeError(f0),
            wlist_native::common::exceptions::UniverseError::Other(f0, ) => UniverseError::Other(f0),
            _ => unreachable!(),
        }
    }
}
