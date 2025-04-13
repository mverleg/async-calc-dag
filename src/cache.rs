use crate::common::{Error, Identifier};
use crate::file::File;
use ::append_only_vec::AppendOnlyVec;
use ::scc::hash_map::Entry;
use crate::lazy_async::ALazy;

// TODO: move this and ALazy (incl scss/appemnd_only_vec) to separate crate later

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

    pub async fn get<F>(&self, f: impl FnOnce() -> F) -> &Result<T, Error>
            where F: Future<Output=Result<T, Error>> {
        self.value.get_or_init(f).await
    }
}

pub async fn read(&self, iden: &Identifier) -> Result<File, Error> {
    // TODO @mverleg:
    // - core must be used across threads, don't take &mut
    // - how to do cache key lookup efficiently? just hashmap for now?
    let file = match self.files.entry(iden.clone()) {
        // ^ this clone is undesirable, but whole data structure will likely be optimized anyway
        Entry::Occupied(occupied) =>
            occupied.get().share(),
        Entry::Vacant(vacant) =>
            vacant.insert_entry(ALazy::new()).get().share(),
    };
    file.get(|| self.fs.read(iden)).await.share()
}
