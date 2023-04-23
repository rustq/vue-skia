static mut MAX_ID: usize = 0;

pub struct Tree {
    root: Option<Box<Node>>
}

pub struct Node {
    pub id: usize,
    node_vec: Vec<Box<Node>>
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: None
        }
    }

    pub fn get_root(&mut self) -> Option<&mut Node> {
        self.root.as_deref_mut().map(|mut root| { root })
    }

    pub fn set_root(&mut self, root: Box<Node>) {
        self.root = Some(root);
    }
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: unsafe {
                MAX_ID += 1;
                MAX_ID
            },
            node_vec: Vec::new()
        }
    }

    pub fn append(&mut self, node: Box<Node>) {
        self.node_vec.push(node);
    }

    pub fn remove(&mut self, node: Box<Node>) {
        self.node_vec.retain(|t| t.id != node.id);
    }

    pub fn get_children_len(&self) -> usize {
        self.node_vec.len()
    }
}

#[cfg(test)]
mod test {
    use super::Tree;
    use super::Node;
    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        let mut root = Node::new();

        assert_eq!(root.id, 1);
        assert_eq!(root.get_children_len(), 0);

        tree.set_root(Box::new(root));

        let node_0 = Box::new(Node::new());
        let node_1 = Box::new(Node::new());
        assert_eq!(node_0.id, 2);
        assert_eq!(node_1.id, 3);

        let root: &mut Node = tree.get_root().unwrap();
        assert_eq!(root.id, 1);
        assert_eq!(root.get_children_len(), 0);
        root.append(node_0);
        assert_eq!(root.get_children_len(), 1);
        root.append(node_1);
        assert_eq!(root.get_children_len(), 2);

        let node_2 = Box::new(Node::new());
        assert_eq!(node_2.id, 4);

        let root: &mut Node = tree.get_root().unwrap();
        root.append(node_2);
        assert_eq!(root.get_children_len(), 3);
    }
}