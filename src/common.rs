use std::fmt;
use std::hash::Hash;
use hipstr::{HipStr, LocalHipStr};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
// TODO: is it useful to cache the hashcode?
pub struct Identifier {
    pub value: LocalHipStr<'static>,
}

impl Identifier {
    pub fn of(value: impl Into<String>) -> Self {
        // should validate input
        Self { value: LocalHipStr::from(value.into()) }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub trait Share {
    fn share(&self) -> Self;
}
// TODO perf: we wouldn't need all the Arcs if we could borrow from Core maps (which wouldn't be safe now, but would be if they were some grow-only no-move vec)

impl <T: Share> Share for Result<T, Error> {
    fn share(&self) -> Self {
        match self {
            Ok(val) => Ok(val.share()),
            Err(err) => Err(err.clone()),
        }
    }
}

/// Has an id used for caching. The id must exactly identify the data.
/// - Using too little will give mismatched caceh hits, causing strange bugs
/// - Using too much is hard since this trait is a projection, but would lead to cache misses
/// - This is used often, so both `id` and the result's hashcode should be cheap
pub trait HasId {
    type Uid: Eq + Hash;

    fn id(&self) -> Self::Uid;
}

impl HasId for Identifier {
    type Uid = LocalHipStr<'static>;

    fn id(&self) -> Self::Uid {
        self.value.clone()
    }
}

impl <'t, A: HasId, B: HasId> HasId for (&'t A, &'t B) {
    type Uid = (A::Uid, B::Uid);

    fn id(&self) -> Self::Uid {
        (self.0.id(), self.1.id())
    }
}

// TODO mverleg: do a macro or something
impl <'t, A: HasId, B: HasId, C: HasId> HasId for (&'t A, &'t B, &'t C) {
    type Uid = (A::Uid, B::Uid, C::Uid);

    fn id(&self) -> Self::Uid {
        (self.0.id(), self.1.id(), self.2.id())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    FileNotFound(Identifier),
    CouldNotParse(Identifier),
    DivideByZero(Identifier, i64),
    NoSuchArg(Identifier, u32),
}
