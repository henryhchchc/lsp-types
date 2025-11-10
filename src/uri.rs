use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{de::Error, Deserialize, Serialize};

/// Newtype struct around `fluent_uri::Uri<String>` with serialization implementations that use `as_str()` and 'from_str()' respectively.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uri(fluent_uri::Uri<String>);

impl From<fluent_uri::Uri<String>> for Uri {
    fn from(uri: fluent_uri::Uri<String>) -> Self {
        Self(uri)
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        fluent_uri::Uri::<String>::parse(string)
            .map(Uri)
            .map_err(|(_error, msg)| Error::custom(msg))
    }
}

impl FromStr for Uri {
    type Err = fluent_uri::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fluent_uri::Uri::from_str(s).map(|uri| uri.into())
    }
}

impl Deref for Uri {
    type Target = fluent_uri::Uri<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uri {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
