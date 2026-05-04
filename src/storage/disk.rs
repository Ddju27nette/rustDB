// ici c'est le module pour gérer l'accès au disque dur, y compris les opérations de lecture et d'écriture de pages.
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

use crate::storage::{PAGE_SIZE};
use crate::storage::page::{Page, PageId};

pub struct DiskManager {
    file: File,
}

impl DiskManager {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        Self { file }
    }
    pub fn read_page(&mut self, page_id: PageId) -> Page {
        let mut page = Page::new(page_id);
        let offset = page_id * PAGE_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        self.file.read(&mut page.data).unwrap_or(0);
        page
    }
    pub fn write_page(&mut self, page: &Page) {
        let offset = page.id * PAGE_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        self.file.write_all(&page.data).unwrap();
    }
    pub fn flush(&mut self) {
        self.file.sync_all().unwrap();
    }
}