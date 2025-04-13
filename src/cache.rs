use std::hash::Hash;
use crate::common::Error;
use crate::common::Identifier;
use crate::file::File;
use crate::lazy_async::ALazy;
use ::append_only_vec::AppendOnlyVec;
use ::scc;

// TODO: add a way to get refs, to prevent repeated hashing (if this ever comes up)
// pub struct Ref<V> {
//     ix: usize,
// }

// TODO: move this and ALazy (incl scss/appemnd_only_vec) to separate crate later

// TODO: should be stored to disk, only data is enough, lookup can be reconstructed
// TODO: cannot be shrunk live, but should have a way to reinitialize without old entries
/// A cache with these properties:
///
/// - Can only grow (to shrink, must replace by a shrunken version)
/// - Elements get initialized once; subsequent initializes can wait (async) for completion
/// - Can borrow any number of elements, incl repeats, because data never moves
///
/// Some assumptions:
/// - Producer does not panic, or the cache isn't looked at after it does
pub struct Cache<K, V> {
    lookup: scc::HashMap<K, usize>,
    data: AppendOnlyVec<ALazy<V>>,
}

impl <K, V> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            lookup: Default::default(),
            data: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl <K: Eq + Hash, V> Cache<K, V> {
    // init itself must be async?
    pub async fn get<F>(&self, key: &K, init: impl FnOnce() -> F) -> &Result<V, Error>
            where F: Future<Output=Result<V, Error>> {
        let ix = match self.lookup.entry(key) {
            scc::hash_map::Entry::Occupied(occupied) => *occupied.get(),
            scc::hash_map::Entry::Vacant(vacant) => self.data.push(ALazy::new()),
        };
        self.data[ix].get(init).await
    }
}

