use gloo_storage::{LocalStorage, Storage};
use serde::{de::Deserialize, ser::Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum StorageError {
    #[error("could not load data from browser local storage")]
    GlooStorageGetError {
        #[from]
        source: gloo_storage::errors::StorageError,
    },
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Store();

impl Store {
    pub(crate) fn get<T>(&self, key: &str) -> Result<Option<T>, StorageError>
    where
        T: for<'de> Deserialize<'de>,
    {
        match LocalStorage::get(key) {
            Ok(v) => Ok(Some(v)),
            Err(e) => Ok(None),
        }
    }

    pub(crate) fn set<T>(&self, key: &str, value: T) -> Result<(), StorageError>
    where
        T: Serialize,
    {
        Ok(LocalStorage::set(key, value)?)
    }
}
