use crate::storage::buffer_pool::BufferPool;
use crate::storage::page::PageId;
use crate::btree::node::{BTreeNode, NodeType};

pub struct BTree {
    root: PageId,
    buffer_pool: BufferPool,
}

impl BTree {
    pub fn new(mut buffer_pool: BufferPool) -> Self {
        let root = 0;

        let page = buffer_pool.fetch_page(root);
        let is_empty = page.data.iter().all(|&b| b == 0);
        if is_empty {
            let node = BTreeNode {
                node_type: NodeType::Leaf,
                keys: vec![],
                values: vec![],
                children: vec![],
            };
            let new_page = node.to_page(root);
            *page = new_page;
        }

        Self { root, buffer_pool }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let page = self.buffer_pool.fetch_page(self.root);
        let mut node = BTreeNode::from_page(page);

        if node.node_type != NodeType::Leaf {
            panic!("Only leaf nodes are supported in this simple implementation");
        }

        if let Some(pos) = node.keys.iter().position(|k| k == &key) {
            node.values[pos] = value;
        } else {
            let pos = node.keys.iter().position(|k| k > &key).unwrap_or(node.keys.len());
            node.keys.insert(pos, key);
            node.values.insert(pos, value);
        }

        let new_page = node.to_page(self.root);
        *page = new_page;
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let page = self.buffer_pool.fetch_page(self.root);
        let node = BTreeNode::from_page(page);
        node.keys.iter().position(|k| k == key).map(|pos| node.values[pos].clone())
    }

    pub fn delete(&mut self, key: &[u8]) {
        let page = self.buffer_pool.fetch_page(self.root);
        let mut node = BTreeNode::from_page(page);

        if let Some(pos) = node.keys.iter().position(|k| k == key) {
            node.keys.remove(pos);
            node.values.remove(pos);
            let new_page = node.to_page(self.root);
            *page = new_page;
        }
    }

    pub fn scan_prefix(&mut self, prefix: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        let page = self.buffer_pool.fetch_page(self.root);
        let node = BTreeNode::from_page(page);

        node.keys
            .iter()
            .zip(node.values.iter())
            .filter(|(key, _)| key.starts_with(prefix))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    pub fn flush(&mut self) {
        self.buffer_pool.flush_all();
    }
}