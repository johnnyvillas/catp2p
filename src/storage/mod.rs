//! Storage functionality for persisting data.

pub mod db;

use crate::error::Error;
use sled::Db;
use std::path::Path;

/// The main storage manager for CatP2P.
pub struct StorageManager {
    db: Db,
}

impl StorageManager {
    /// Creates a new StorageManager with the given database path.
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, Error> {
        let db = sled::open(db_path)
            .map_err(|e| Error::Storage(format!("Failed to open database: {}", e)))?;
        
        Ok(Self {
            db,
        })
    }

    /// Stores a key-value pair.
    pub fn put<K, V>(&self, key: K, value: V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.db.insert(key, value)
            .map_err(|e| Error::Storage(format!("Failed to store data: {}", e)))?;
        self.db.flush()
            .map_err(|e| Error::Storage(format!("Failed to flush database: {}", e)))?;
        Ok(())
    }

    /// Retrieves a value by key.
    pub fn get<K>(&self, key: K) -> Result<Option<Vec<u8>>, Error>
    where
        K: AsRef<[u8]>,
    {
        self.db.get(key)
            .map(|opt| opt.map(|ivec| ivec.to_vec()))
            .map_err(|e| Error::Storage(format!("Failed to retrieve data: {}", e)))
    }

    /// Removes a key-value pair.
    pub fn remove<K>(&self, key: K) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
    {
        self.db.remove(key)
            .map_err(|e| Error::Storage(format!("Failed to remove data: {}", e)))?;
        self.db.flush()
            .map_err(|e| Error::Storage(format!("Failed to flush database: {}", e)))?;
        Ok(())
    }

    /// Closes the database.
    pub fn close(self) -> Result<(), Error> {
        self.db.flush()
            .map_err(|e| Error::Storage(format!("Failed to flush database: {}", e)))?;
        Ok(())
    }
}
