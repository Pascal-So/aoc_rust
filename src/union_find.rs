/// [Union-Find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) data
/// structure where every set has some associated data.
#[derive(Debug, Clone)]
pub struct UnionFind<T> {
    // Misusing [std::result::Result] as a general purpose Either type.
    data: Vec<Result<T, SetIndex>>,
}

impl<T> UnionFind<T> {
    /// Constructs a new, empty [UnionFind<T>].
    pub fn new() -> UnionFind<T> {
        UnionFind { data: Vec::new() }
    }

    /// Constructs a new, empty [UnionFind<T>] with the specified capacity.
    pub fn with_capacity(size: usize) -> UnionFind<T> {
        UnionFind {
            data: Vec::with_capacity(size),
        }
    }

    /// Add a new set to the structure.
    pub fn new_set(&mut self, data: T) -> SetIndex {
        let set = SetIndex(self.data.len());
        self.data.push(Ok(data));
        set
    }

    /// Merge two sets, using the provided function to combine the
    /// data for the two sets.
    pub fn merge_with(&mut self, a: SetIndex, b: SetIndex, merger: impl FnOnce(T, T) -> T) {
        let root_a = self.lookup_root_optimize(a);
        let root_b = self.lookup_root_optimize(b);

        if root_a == root_b {
            return;
        }

        // We use temporary values so that we can swap the associated data out
        // of the data vec without copying. Both of these entires are overwritten
        // later, therefore the temporary values will never show up for the user.
        let mut data_a = Err(SetIndex(usize::MAX));
        let mut data_b = Err(SetIndex(usize::MAX));
        std::mem::swap(&mut self.data[root_a.0], &mut data_a);
        std::mem::swap(&mut self.data[root_b.0], &mut data_b);
        let data = merger(data_a.unwrap(), data_b.unwrap());

        self.data[root_a.0] = Err(root_b);
        self.data[root_b.0] = Ok(data);
    }

    fn lookup_root(&self, mut set: SetIndex) -> SetIndex {
        while let Err(parent) = self.data[set.0] {
            set = parent;
        }
        set
    }

    fn lookup_root_optimize(&mut self, mut set: SetIndex) -> SetIndex {
        let mut stack = Vec::new();

        while let Err(parent) = self.data[set.0] {
            stack.push(set);
            set = parent;
        }

        // the last entry on [stack] already points to the correct parent.
        for s in &stack[..stack.len().saturating_sub(1)] {
            self.data[s.0] = Err(set);
        }

        set
    }

    /// Compact all parent pointer datastructure in order to
    /// make future lookups faster. After this, every parent
    /// pointer directly points to the root of the set: either
    /// to itself or to the root index.
    pub fn optimize(&mut self) {
        for i in 0..self.data.len() {
            self.lookup_root_optimize(SetIndex(i));
        }
    }

    /// Get an immutable reference to the data associated with a set.
    pub fn get(&self, set: SetIndex) -> &T {
        self.data[self.lookup_root(set).0].as_ref().unwrap()
    }

    /// Get a mutable reference to the data associated with a set.
    pub fn get_mut(&mut self, set: SetIndex) -> &mut T {
        let idx = self.lookup_root_optimize(set).0;
        self.data[idx].as_mut().unwrap()
    }
}

impl<T> Default for UnionFind<T> {
    fn default() -> Self {
        UnionFind::new()
    }
}

impl<'a, T> IntoIterator for &'a UnionFind<T> {
    type Item = <UnionFindIterator<'a, T> as Iterator>::Item;
    type IntoIter = UnionFindIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        UnionFindIterator {
            uf: self,
            idx: Some(SetIndex(0)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SetIndex(usize);

pub struct UnionFindIterator<'a, T> {
    uf: &'a UnionFind<T>,
    idx: Option<SetIndex>,
}

impl<'a, T> UnionFindIterator<'a, T> {
    pub fn advance(&mut self) {
        if let Some(SetIndex(mut idx)) = self.idx {
            idx += 1;
            while idx < self.uf.data.len() && self.uf.data[idx].is_err() {
                idx += 1;
            }
            self.idx = if idx >= self.uf.data.len() {
                None
            } else {
                Some(SetIndex(idx))
            };
        }
    }
}

impl<'a, T> Iterator for UnionFindIterator<'a, T> {
    type Item = (SetIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.idx {
            if self.uf.data[idx.0].is_err() {
                self.advance();
            }
        }
        if let Some(idx) = self.idx {
            let res = (idx, self.uf.get(idx));
            self.advance();
            return Some(res);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new();
        let idx_5 = uf.new_set(5);
        let idx_8 = uf.new_set(8);
        let idx_9 = uf.new_set(9);

        uf.merge_with(idx_5, idx_9, i32::max);

        assert_eq!(*uf.get(idx_5), 9);
        assert_eq!(*uf.get(idx_8), 8);
        assert_eq!(*uf.get(idx_9), 9);

        let mut vec: Vec<_> = uf.into_iter().collect();
        assert_eq!(vec.len(), 2);
        if *vec[0].1 == 9 {
            vec.reverse();
        }

        assert_eq!(vec[0], (idx_8, &8));
        assert_eq!(vec[1].1, &9);
        assert!(vec[1].0 == idx_5 || vec[1].0 == idx_9);
    }
}
