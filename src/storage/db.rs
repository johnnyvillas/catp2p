/* Copyright 2025 Joao Guimaraes, Catp2p Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Database functionality for persisting data.

use crate::error::Error;
use sled::Db;
use std::path::Path;

/// A key-value database wrapper.
pub struct Database {
    db: Db,
}

impl Database {
    /// Opens a database at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let db = sled::open(path)
            .map_err(|e| Error::Storage(format!("Failed to open database: {}", e)))?;
        
        Ok(Self {
            db,
        })
    }
    
    /// Stores a key-value pair.
    pub fn put<K, V>(&self, key: K, value: V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]> + Into<sled::IVec>,
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
    
    /// Creates a new tree (namespace) in the database.
    pub fn open_tree<T>(&self, name: T) -> Result<Tree, Error>
    where
        T: AsRef<[u8]>,
    {
        let tree = self.db.open_tree(name)
            .map_err(|e| Error::Storage(format!("Failed to open tree: {}", e)))?;
        
        Ok(Tree {
            tree,
        })
    }
    
    /// Closes the database.
    pub fn close(self) -> Result<(), Error> {
        self.db.flush()
            .map_err(|e| Error::Storage(format!("Failed to flush database: {}", e)))?;
        Ok(())
    }
}

/// A tree (namespace) in the database.
pub struct Tree {
    tree: sled::Tree,
}

impl Tree {
    /// Stores a key-value pair in the tree.
    pub fn put<K, V>(&self, key: K, value: V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]> + Into<sled::IVec>,
    {
        self.tree.insert(key, value)
            .map_err(|e| Error::Storage(format!("Failed to store data: {}", e)))?;
        self.tree.flush()
            .map_err(|e| Error::Storage(format!("Failed to flush tree: {}", e)))?;
        Ok(())
    }
    
    /// Retrieves a value by key from the tree.
    pub fn get<K>(&self, key: K) -> Result<Option<Vec<u8>>, Error>
    where
        K: AsRef<[u8]>,
    {
        self.tree.get(key)
            .map(|opt| opt.map(|ivec| ivec.to_vec()))
            .map_err(|e| Error::Storage(format!("Failed to retrieve data: {}", e)))
    }
    
    /// Removes a key-value pair from the tree.
    pub fn remove<K>(&self, key: K) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
    {
        self.tree.remove(key)
            .map_err(|e| Error::Storage(format!("Failed to remove data: {}", e)))?;
        self.tree.flush()
            .map_err(|e| Error::Storage(format!("Failed to flush tree: {}", e)))?;
        Ok(())
    }
}
