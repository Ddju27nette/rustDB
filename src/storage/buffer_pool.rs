// c'est le fichier qui contient le module pour gérer le pool de mémoire, qui est une partie importante du système de stockage. Le pool de mémoire est utilisé pour stocker les pages de données en mémoire, ce qui permet d'accélérer les opérations de lecture et d'écriture en évitant d'avoir à accéder au disque dur à chaque fois. Le module buffer_pool va gérer la mise en cache des pages, la gestion de la mémoire, et les politiques d'éviction des pages lorsque le pool de mémoire est plein. 
use std::collections::{HashMap, VecDeque};

use crate::storage::disk::DiskManager;
use crate::storage::page::{Page, PageId};

pub struct BufferPool{
    capacity: usize,
    pages: HashMap<PageId, Page>,
    lru: VecDeque<PageId>,
    pub disk: DiskManager,
}
impl BufferPool {
    pub fn new(capacity: usize, disk: DiskManager) -> Self {
        Self {
            capacity,
            pages: HashMap::new(),
            lru: VecDeque::new(),
            disk,
        }
    }
    pub fn fetch_page(&mut self, page_id: PageId) -> &mut Page {
        if ! self.pages.contains_key(&page_id) {
             if self.pages.len() == self.capacity {
                self.evict();
                }
            let page = self.disk.read_page(page_id);
            self.pages.insert(page_id, page);
        }
        self.touch(page_id);
        self.pages.get_mut(&page_id).unwrap()
    }
    fn touch(&mut self, id: PageId) {
        self.lru.retain(|&x| x != id);
        self.lru.push_back(id);
    }
    fn evict(&mut self) {
        if let Some(old) = self.lru.pop_front() {
            if let Some(page) = self.pages.remove(&old) {
                if page.is_dirty {
                    self.disk.write_page(&page);
                }
            }
        }
    }
    pub fn flush_all(&mut self){
        for page in self.pages.values(){
            if page.is_dirty{
                self.disk.write_page(page);
            }
        }
        self.disk.flush();
    }
        
}
