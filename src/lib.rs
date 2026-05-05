// API
pub mod storage;
pub mod btree;
pub mod wal;
pub mod tx;
// pub mod common;

use crate::{
    btree::tree::BTree,
    storage::{buffer_pool::BufferPool, disk::DiskManager},
    tx::{Transaction, TransactionManager},
    wal::WAL,
};

pub struct RustDB {
    tree: BTree,
    secondary_index: BTree,
    wal: WAL,
    tx_manager: TransactionManager,
}

impl RustDB {
    pub fn open() -> Self {
        RustDB::open_with_path("db.db")
    }

    pub fn open_with_path(path: &str) -> Self {
        let disk = DiskManager::new(path);
        let buffer = BufferPool::new(10, disk);
        let tree = BTree::new(buffer);

        let index_path = format!("{}.idx", path);
        let index_disk = DiskManager::new(&index_path);
        let index_buffer = BufferPool::new(10, index_disk);
        let secondary_index = BTree::new(index_buffer);

        let wal_path = format!("{}.wal", path);
        let wal = WAL::new(&wal_path);

        let mut db = Self {
            tree,
            secondary_index,
            wal,
            tx_manager: TransactionManager::new(),
        };

        db.recover();
        db
    }

    fn recover(&mut self) {
        let mut entries = Vec::new();
        self.wal.replay(|entry| entries.push(entry));

        for entry in entries {
            match entry {
                crate::wal::WALEntry::Put(key, value) => {
                    self.tree.insert(key, value);
                }
                crate::wal::WALEntry::Delete(key) => {
                    self.tree.delete(&key);
                }
                crate::wal::WALEntry::IndexPut(index_key, primary_key) => {
                    self.secondary_index.insert(index_key, primary_key);
                }
                crate::wal::WALEntry::IndexDelete(index_key) => {
                    self.secondary_index.delete(&index_key);
                }
            }
        }

        self.tree.flush();
        self.secondary_index.flush();
        self.wal.checkpoint();
    }

    fn checkpoint(&mut self) {
        self.tree.flush();
        self.secondary_index.flush();
        self.wal.checkpoint();
    }

    // ✔ utile pour tests
    pub fn manual_checkpoint(&mut self) {
        self.checkpoint();
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.wal.log_put(&key, &value);
        self.tree.insert(key, value);
        self.checkpoint(); // ✔ IMPORTANT
    }

    pub fn put_with_secondary(&mut self, key: Vec<u8>, value: Vec<u8>, secondary_key: Vec<u8>) {
        self.wal.log_put(&key, &value);
        self.wal.log_index_put(&secondary_key, &key);
        self.tree.insert(key.clone(), value);
        self.secondary_index.insert(secondary_key, key);
        self.checkpoint(); // ✔ IMPORTANT
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.tree.get(key)
    }

    pub fn get_by_secondary(&mut self, secondary_key: &[u8]) -> Option<Vec<u8>> {
        self.secondary_index
            .get(secondary_key)
            .and_then(|primary| self.tree.get(&primary))
    }

    pub fn scan(&mut self, prefix: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        self.tree.scan_prefix(prefix)
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.wal.log_delete(key);
        self.tree.delete(key);
        self.checkpoint(); // ✔ IMPORTANT
    }

    pub fn delete_with_secondary(&mut self, key: &[u8], secondary_key: &[u8]) {
        self.wal.log_delete(key);
        self.wal.log_index_delete(secondary_key);
        self.tree.delete(key);
        self.secondary_index.delete(secondary_key);
        self.checkpoint(); // ✔ IMPORTANT
    }

    // ✔ CORRECTION ICI
    pub fn begin_transaction(&mut self) -> Transaction<'_> {
        let lock = self.tx_manager.get_lock();
        Transaction::new(self, lock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let _ = std::fs::remove_file("test.db");
        let mut db = RustDB::open_with_path("test.db");
        db.put(b"a".to_vec(), b"1".to_vec());
        assert_eq!(db.get(b"a"), Some(b"1".to_vec()));
    }

    #[test]
    fn test_delete() {
        let _ = std::fs::remove_file("test.db");
        let mut db = RustDB::open_with_path("test.db");
        db.put(b"key1".to_vec(), b"value1".to_vec());
        db.delete(b"key1");
        assert_eq!(db.get(b"key1"), None);
    }
}