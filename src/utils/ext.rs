pub trait HashSetExt<T> {
    /// Inserts [`value`] if it does not exist in the set. Otherwise, removes the existing one.
    /// If so, returns first the removed one, then the [`value`].
    fn toggle(&mut self, value: T) -> Option<(T, T)>;
}

impl<T: Eq + std::hash::Hash> HashSetExt<T> for std::collections::HashSet<T> {
    fn toggle(&mut self, value: T) -> Option<(T, T)> {
        if let Some(removed) = self.take(&value) {
            Some((removed, value))
        } else {
            self.insert(value);
            None
        }
    }
}
