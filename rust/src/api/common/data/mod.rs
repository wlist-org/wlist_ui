pub mod storages;
pub mod files;
pub mod trashes;

#[flutter_rust_bridge::frb(non_opaque)]
/// The order direction.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::Direction)]
pub enum FDirection {
    /// Ascending order.
    ASCEND,
    /// Descending order.
    DESCEND,
}

#[flutter_rust_bridge::frb(opaque)]
/// An Either can be either be a "Left", containing an LHS value or a "Right" containing an RHS value,
/// but it cannot be "neither" nor "both".
pub struct FEither<L, R> {
    either: either::Either<L, R>,
}

impl<A, B, L, R> From<either::Either<A, B>> for FEither<L, R> where A: Into<L>, B: Into<R> {
    #[inline]
    fn from(value: either::Either<A, B>) -> Self {
        Self { either: value.map_either(Into::into, Into::into) }
    }
}

impl<L, R> FEither<L, R> {
    /// Get the left value.
    #[inline]
    pub fn left(&self) -> Option<&L> {
        self.either.as_ref().left()
    }

    /// Get the right value.
    #[inline]
    pub fn right(&self) -> Option<&R> {
        self.either.as_ref().right()
    }
}
