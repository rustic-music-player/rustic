use std::sync::Arc;

use bincode::deserialize;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use serde::de::DeserializeOwned;

pub fn serialize_id(id: usize) -> Result<Vec<u8>, Error> {
    let mut id_bytes = Vec::new();
    id_bytes.write_u64::<LittleEndian>(id as u64)?;

    Ok(id_bytes)
}

pub fn deserialize_id(id: &[u8]) -> Result<usize, Error> {
    let mut bytes = id.clone();
    let id = bytes.read_u64::<LittleEndian>()?;

    Ok(id as usize)
}

pub fn fetch_entity<E>(tree: &Arc<sled::Tree>, id: usize) -> Result<Option<E>, Error>
where
    E: DeserializeOwned,
{
    let id = serialize_id(id)?;
    if let Some(bytes) = tree.get(&id)? {
        let entity: E = deserialize(&bytes)?;
        Ok(Some(entity))
    } else {
        Ok(None)
    }
}

pub fn fetch_entities<E>(tree: &Arc<sled::Tree>) -> Result<Vec<E>, Error>
where
    E: DeserializeOwned,
{
    tree.iter()
        .map(|item| {
            item.map_err(Error::from)
                .and_then(|(_, bytes)| deserialize(&bytes).map_err(Error::from))
        })
        .collect()
}

pub fn search_entities<E, P>(tree: &Arc<sled::Tree>, predicate: P) -> Result<Vec<E>, Error>
where
    E: DeserializeOwned,
    P: Fn(&E) -> bool,
{
    tree.iter()
        .map(|item| {
            item.map_err(Error::from)
                .and_then(|(_, bytes)| deserialize(&bytes).map_err(Error::from))
        })
        .filter(|item| match item {
            Ok(entity) => predicate(entity),
            _ => false,
        })
        .collect()
}

pub fn find_entity<E, M>(tree: &Arc<sled::Tree>, matches: M) -> Result<Option<E>, Error>
where
    E: DeserializeOwned,
    M: Fn(&E) -> bool,
{
    tree.iter()
        .map(|item| {
            item.map_err(Error::from).and_then(|(_, bytes)| {
                let entity: E = deserialize(&bytes)?;
                Ok(entity)
            })
        })
        .find(|item| match item {
            Ok(t) => matches(t),
            _ => false,
        })
        .transpose()
}
