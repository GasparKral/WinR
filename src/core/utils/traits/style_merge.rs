pub trait StyleMerge {
    /// Combina este estilo con otro, donde `other` tiene precedencia sobre `self`
    fn merge_with(&self, other: &Self) -> Self;
}
