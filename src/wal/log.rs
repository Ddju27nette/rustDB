use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

pub enum WALEntry {
    Put(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
    IndexPut(Vec<u8>, Vec<u8>),
    IndexDelete(Vec<u8>),
}

pub struct WAL {
    file: std::fs::File,
    path: String,
}

impl WAL {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

        Self {
            file,
            path: path.to_string(),
        }
    }

    pub fn log_put(&mut self, key: &[u8], value: &[u8]) {
        self.file.write_all(&[1]).unwrap();
        self.file.write_all(&(key.len() as u32).to_le_bytes()).unwrap();
        self.file.write_all(&(value.len() as u32).to_le_bytes()).unwrap();
        self.file.write_all(key).unwrap();
        self.file.write_all(value).unwrap();
    }

    pub fn log_delete(&mut self, key: &[u8]) {
        self.file.write_all(&[2]).unwrap();
        self.file.write_all(&(key.len() as u32).to_le_bytes()).unwrap();
        self.file.write_all(key).unwrap();
    }

    pub fn log_index_put(&mut self, index_key: &[u8], primary_key: &[u8]) {
        self.file.write_all(&[3]).unwrap();
        self.file.write_all(&(index_key.len() as u32).to_le_bytes()).unwrap();
        self.file.write_all(&(primary_key.len() as u32).to_le_bytes()).unwrap();
        self.file.write_all(index_key).unwrap();
        self.file.write_all(primary_key).unwrap();
    }

    pub fn log_index_delete(&mut self, index_key: &[u8]) {
        self.file.write_all(&[4]).unwrap();
        self.file.write_all(&(index_key.len() as u32).to_le_bytes()).unwrap();
        self.file.write_all(index_key).unwrap();
    }

    pub fn replay<F>(&mut self, mut visitor: F)
    where
        F: FnMut(WALEntry),
    {
        self.file.seek(SeekFrom::Start(0)).unwrap();

        loop {
            let mut op = [0u8; 1];
            if self.file.read_exact(&mut op).is_err() {
                break;
            }

            match op[0] {
                1 => {
                    let key_len = self.read_u32() as usize;
                    let value_len = self.read_u32() as usize;
                    let key = self.read_bytes(key_len);
                    let value = self.read_bytes(value_len);
                    visitor(WALEntry::Put(key, value));
                }
                2 => {
                    let key_len = self.read_u32() as usize;
                    let key = self.read_bytes(key_len);
                    visitor(WALEntry::Delete(key));
                }
                3 => {
                    let index_key_len = self.read_u32() as usize;
                    let primary_key_len = self.read_u32() as usize;
                    let index_key = self.read_bytes(index_key_len);
                    let primary_key = self.read_bytes(primary_key_len);
                    visitor(WALEntry::IndexPut(index_key, primary_key));
                }
                4 => {
                    let index_key_len = self.read_u32() as usize;
                    let index_key = self.read_bytes(index_key_len);
                    visitor(WALEntry::IndexDelete(index_key));
                }
                _ => break,
            }
        }

        self.file.seek(SeekFrom::End(0)).unwrap();
    }

    pub fn checkpoint(&mut self) {
        self.file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();

        self.file.flush().unwrap();
    }

    fn read_u32(&mut self) -> u32 {
        let mut buffer = [0u8; 4];
        self.file.read_exact(&mut buffer).unwrap();
        u32::from_le_bytes(buffer)
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; len];
        self.file.read_exact(&mut buffer).unwrap();
        buffer
    }
}