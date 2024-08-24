#![allow(dead_code, non_snake_case)]

use std::borrow::Cow;
use std::sync::Arc;

use wlist_native::common::data::files::FileLocation;
use hashbrown::HashMap;
pub use wlist_native::common::exceptions::*;

macro_rules! define_exception {
    ($exception: ident $origin: ident() $msg: literal) => {
        #[::flutter_rust_bridge::frb(mirror($origin), opaque)]
        pub struct $exception {}
        #[flutter_rust_bridge::frb(sync, getter)]
        pub fn $origin() -> $origin {
            unreachable!()
        }
    };
    ($exception: ident $origin: ident($($field: ident: $ty: ty),+) |&$self: ident, $f: ident| $write: expr) => {
        #[::flutter_rust_bridge::frb(mirror($origin), opaque)]
        pub struct $exception {
            $($field: $ty),+
        }
        #[flutter_rust_bridge::frb(sync, getter)]
        pub fn $origin() -> $origin {
            unreachable!()
        }
    };
}

define_exception!(_UnavailableApiVersionError UnavailableApiVersionError() "current api version is unavailable");
define_exception!(_MatchFrequencyControlError MatchFrequencyControlError() "match frequency control");
define_exception!(_IncorrectArgumentError IncorrectArgumentError(e: Cow<'static, str>) |&e, f| write!(f, "incorrect argument: {}", e.e));
define_exception!(_TooLargeDataError TooLargeDataError() "too large data");
define_exception!(_PasswordMismatchedError PasswordMismatchedError() "password is mismatched");
define_exception!(_TokenExpiredError TokenExpiredError() "token is expired");

define_exception!(_InvalidStorageConfigError InvalidStorageConfigError(e: HashMap<String, String>) |&e, f| write!(f, "invalid storage config: {:?}", e.e));
define_exception!(_IncorrectStorageAccountError IncorrectStorageAccountError() "incorrect storage account");
define_exception!(_DuplicateStorageError DuplicateStorageError() "duplicate storage");
define_exception!(_StorageNotFoundError StorageNotFoundError() "storage not found");
define_exception!(_StorageTypeMismatchedError StorageTypeMismatchedError() "storage type is mismatched");
define_exception!(_StorageInLockError StorageInLockError(e: Cow<'static, str>) |&e, f| write!(f, "storage in lock: {}", e.e));

define_exception!(_ComplexOperationError ComplexOperationError() "operation is too complex");
define_exception!(_DuplicateFileError DuplicateFileError(l: FileLocation, name: Arc<String>) |&e, f| write!(f, "duplicate file {} in {:?}", e.name, e.l));
define_exception!(_FileNotFoundError FileNotFoundError(l: FileLocation) |&e, f| write!(f, "file not found {:?}", e.l));
define_exception!(_FileInLockError FileInLockError(l: FileLocation, e: Cow<'static, str>) |&e, f| write!(f, "{} (locking {:?})", e.e, e.l));
define_exception!(_UploadChunkIncompleteError UploadChunkIncompleteError() "upload chunks are incomplete");

define_exception!(_IllegalSuffixError IllegalSuffixError(suffix: Cow<'static, str>) |&e, f| write!(f, "illegal suffix: {}", e.suffix));
define_exception!(_InvalidFilenameError InvalidFilenameError(name: Arc<String>, ch: Option<char>) |&e, f| write!(f, "invalid filename: {} char: {:?}", e.name, e.ch));
define_exception!(_NameTooLongError NameTooLongError(name: Arc<String>, max: Option<u64>) |&e, f| write!(f, "name too long: {} limit: {:?}", e.name, e.max));
define_exception!(_ReadOnlyStorageError ReadOnlyStorageError() "readonly storage");
define_exception!(_SpaceNotEnoughError SpaceNotEnoughError(left: Option<u64>, need: Option<u64>) |&e, f| write!(f, "space not enough: {:?} < {:?}", e.left, e.need));
define_exception!(_FlowNotEnoughError FlowNotEnoughError(upload: bool, left: Option<u64>, need: Option<u64>) |&e, f| write!(f, "{} flow not enough: {:?} < {:?}", if e.upload { "upload" } else { "download" }, e.left, e.need));
define_exception!(_FileTooLargeError FileTooLargeError(size: u64, max: Option<u64>) |&e, f| write!(f, "file too large: {} limit: {:?}", e.size, e.max));
