use std::sync::{Arc, Mutex};

use crate::RustDB;

#[derive(Clone)]
pub struct TransactionManager {
    lock: Arc<Mutex<()>>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            lock: Arc::new(Mutex::new(())),
        }
    }

    //  CETTE FONCTION MANQUAIT
    pub fn get_lock(&self) -> Arc<Mutex<()>> {
        Arc::clone(&self.lock)
    }
}

pub enum Operation {
    Put(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}

pub struct Transaction<'a> {
    db: &'a mut RustDB,
    lock: Arc<Mutex<()>>,
    pending: Vec<Operation>,
    active: bool,
}

impl<'a> Transaction<'a> {
    pub fn new(db: &'a mut RustDB, lock: Arc<Mutex<()>>) -> Self {
    Self {
        db,
        lock,
        pending: Vec::new(),
        active: true,
    }
    }
    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.pending.push(Operation::Put(key, value));
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.pending.push(Operation::Delete(key.to_vec()));
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        for op in self.pending.iter().rev() {
            match op {
                Operation::Put(k, v) if k == key => return Some(v.clone()),
                Operation::Delete(k) if k == key => return None,
                _ => {}
            }
        }

        self.db.tree.get(key)
    }

    pub fn commit(mut self) {
        for op in self.pending.drain(..) {
            match op {
                Operation::Put(key, value) => {
                    self.db.wal.log_put(&key, &value);
                    self.db.tree.insert(key, value);
                }
                Operation::Delete(key) => {
                    self.db.wal.log_delete(&key);
                    self.db.tree.delete(&key);
                    let _guard = self.lock.lock().unwrap();
                }
            }
        }

        self.db.manual_checkpoint();
        self.active = false;
    }

    pub fn rollback(mut self) {
        self.pending.clear();
        self.active = false;
    }
}

    


impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.active {
            self.pending.clear();
        }
    }
}