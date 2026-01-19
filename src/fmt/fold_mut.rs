/// Extension trait for iterators that provides `fold_mut` for chaining with mutable targets.
pub(super) trait IteratorFoldMut: Iterator + Sized {
    /// Folds an iterator into a mutable target, returning the target.
    ///
    /// This is useful for builder patterns where you want to apply multiple operations
    /// to a mutable reference and continue chaining.
    fn fold_mut<T, F>(self, target: &mut T, mut f: F) -> &mut T
    where
        F: FnMut(&mut T, Self::Item),
    {
        self.for_each(|item| f(target, item));
        target
    }
}

impl<I: Iterator> IteratorFoldMut for I {}
