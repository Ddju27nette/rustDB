use crate::storage::page::{Page, PageId};

// # c'est le module qui contient la définition de la structure de données pour les nœuds de l'arbre B, ainsi que les fonctions pour convertir entre les nœuds et les pages de données utilisées par le système de stockage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType{
    Leaf = 0,
    Internal = 1,
}

#[derive(Debug)]
pub struct BTreeNode{
    pub node_type: NodeType,
    pub keys: Vec<Vec<u8>>,
    pub values: Vec<Vec<u8>>,
    pub children: Vec<PageId>,
}
impl BTreeNode{
    pub fn empty_leaf() -> Self {
        Self {
            node_type: NodeType::Leaf,
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn to_page(&self, page_id: PageId) -> Page{
        let mut page = Page::new(page_id);
        let mut offset = 0;
        page.data[offset] = self.node_type as u8;
        offset += 1;

        page.data[offset] = self.keys.len() as u8;
        offset += 1;

        for (key, value) in self.keys.iter().zip(self.values.iter()) {
            let key_len = key.len() as u16;
            let value_len = value.len() as u16;
            let required = offset + 2 + key_len as usize + 2 + value_len as usize;
            if required > page.data.len() {
                panic!("Clé ou valeur trop longue pour tenir dans une page");
            }

            page.data[offset..offset + 2].copy_from_slice(&key_len.to_le_bytes());
            offset += 2;
            page.data[offset..offset + key_len as usize].copy_from_slice(key);
            offset += key_len as usize;

            page.data[offset..offset + 2].copy_from_slice(&value_len.to_le_bytes());
            offset += 2;
            page.data[offset..offset + value_len as usize].copy_from_slice(value);
            offset += value_len as usize;
        }
        page.mark_dirty();
        page
    }
    pub fn from_page(page: &Page) -> Self{
        let mut offset = 0;
        let node_type = match page.data.get(0) {
            Some(&0) => NodeType::Leaf,
            Some(&1) => NodeType::Internal,
            _ => return Self::empty_leaf(),
        };
        offset += 1;

        let num_keys = match page.data.get(offset) {
            Some(&n) => n as usize,
            None => return Self::empty_leaf(),
        };
        offset += 1;

        let mut keys = Vec::new();
        let mut values = Vec::new();

        for _ in 0..num_keys{
            if offset + 2 > page.data.len() {
                return Self::empty_leaf();
            }
            let key_len = u16::from_le_bytes([page.data[offset], page.data[offset + 1]]) as usize;
            offset += 2;
            if offset + key_len > page.data.len() {
                return Self::empty_leaf();
            }
            let key = page.data[offset..offset + key_len].to_vec();
            offset += key_len;
            keys.push(key);

            if offset + 2 > page.data.len() {
                return Self::empty_leaf();
            }
            let value_len = u16::from_le_bytes([page.data[offset], page.data[offset + 1]]) as usize;
            offset += 2;
            if offset + value_len > page.data.len() {
                return Self::empty_leaf();
            }
            let value = page.data[offset..offset + value_len].to_vec();
            offset += value_len;
            values.push(value);
        }
        Self { 
            node_type, 
            keys, 
            values,
            children: Vec::new(),
         }
    }
}