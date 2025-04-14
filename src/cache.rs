use std::hash::Hash;
use crate::common::{CacheId, Error, Share};
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

// TODO: avoid the ToOwned bound? need a method on scc::HashMap that takes entry reference and does copy itself only if needed
impl <K: Eq + Hash, V> Cache<K, V> {
    // init itself must be async?
    /// Look for a previous computed value:
    /// - If it has already been computed, return it
    /// - If it is currently being computed, wait (aync)
    /// - Otherwise (not computed yet), compute it now - only in this case is `init` called
    pub async fn get<A, F>(&self, args: &A, init: fn(&A) -> F) -> &Result<V, Error>
            where A: CacheId<Uid=K>, F: Future<Output=Result<V, Error>> {
        // TODO: how to statically ensure that F only depends on `key`? or is that core's job?
        let ix = *match self.lookup.entry(args.id()) {
            scc::hash_map::Entry::Occupied(occupied) => occupied,
            scc::hash_map::Entry::Vacant(vacant) =>
                vacant.insert_entry(self.data.push(ALazy::new())),
        }.get();
        self.data[ix].get(|| init(args)).await
    }
}

