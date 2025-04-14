use std::fmt;
use std::hash::Hash;
use hipstr::{HipStr, LocalHipStr};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

pub trait CacheId {
    type Uid: Eq + Hash;

    fn id(&self) -> Self::Uid;
}

#[derive(Debug, Clone)]
pub enum Error {
    FileNotFound(Identifier),
    CouldNotParse(Identifier),
    DivideByZero(Identifier, i64),
    NoSuchArg(Identifier, u32),
}
