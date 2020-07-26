use crate::node::Node;
use crate::types::Result;
use getset::Getters;
use std::collections::BTreeMap;
use tokio::sync::Mutex;
use tracing::debug;

// Masks the implementation of our internal map with an async interface
#[derive(Getters, Debug)]
pub struct Nodestore {
    #[getset(get = "pub")]
    self_node: Mutex<Node>,
    main_store: Mutex<BTreeMap<usize, Node>>,
}

impl Nodestore {
    pub fn new(self_node: Node) -> Self {
        let mut node_store = BTreeMap::<usize, Node>::new();
        node_store.insert(*self_node.id(), self_node);
        Nodestore {
            self_node: Mutex::new(self_node),
            main_store: Mutex::new(node_store),
        }
    }

    pub async fn insert(&self, node: Node) -> Result<()> {
        // Get lock
        let mut lock = self.main_store.lock().await;
        lock.insert(*node.id(), node);
        Ok(())
    }

    pub async fn get(&self, id: usize) -> Result<Option<Node>> {
        let lock = self.main_store.lock().await;
        Ok(lock.get(&id).map(ToOwned::to_owned))
    }

    pub async fn get_all(&self) -> Vec<Node> {
        let lock = self.main_store.lock().await;
        lock.values().map(ToOwned::to_owned).collect()
    }

    pub async fn update_from_vec(&self, nodes: Vec<Node>) -> Result<()> {
        let mut lock = self.main_store.lock().await;
        for node in nodes {
            if let Some(i) = lock.get(node.id()) {
                if i.incarnation() < node.incarnation() {
                    debug!("updating entry for node: {} -> {}", i, node);
                    lock.insert(*node.id(), node);
                }
            } else {
                debug!("node did not exist, inserting node: {}", node);
                lock.insert(*node.id(), node);
            }
        }
        Ok(())
    }

    pub async fn len(&self) -> Result<usize> {
        let lock = self.main_store.lock().await;
        Ok(lock.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_store() {
        let self_node = Node::new(0, "1.2.3.4:1000".parse().unwrap());
        let node_store = Nodestore::new(self_node);

        assert_eq!(1, node_store.len().await.unwrap());

        assert_eq!(
            self_node,
            node_store
                .get(*node_store.self_node().lock().await.id())
                .await
                .unwrap()
                .unwrap(),
        );

        let another_node = Node::new(1, "2.3.4.5:1001".parse().unwrap());
        node_store.insert(another_node).await.unwrap();

        assert_eq!(2, node_store.len().await.unwrap());
    }
}
