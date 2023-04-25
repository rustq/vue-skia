static mut MAX_ID: usize = 0;

pub struct Tree {
    root: Option<Box<Node>>
}

pub struct Node {
    pub id: usize,
    value: i32,
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

    pub fn set_root(&mut self, root: Node) {
        self.root = Some(Box::new(root));
    }
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: unsafe {
                MAX_ID += 1;
                MAX_ID
            },
            value: 0,
            node_vec: Vec::new()
        }
    }

    pub fn append(&mut self, node: Node) {
        self.node_vec.push(Box::new(node));
    }

    pub fn append_boxed(&mut self, boxed_node: Box<Node>) {
        self.node_vec.push(boxed_node);
    }

    pub fn remove_by_id(&mut self, node_id: usize) {
        self.node_vec.retain(|t| t.id != node_id);
    }

    pub fn get_children_len(&self) -> usize {
        self.node_vec.len()
    }

    pub fn get_child_by_index(&mut self, index: usize) -> Option<&mut Node> {
        Some(self.node_vec[index].as_mut())
    }

    pub fn update_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn get_value(&mut self) -> i32 {
        self.value
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

        tree.set_root(root);

        let node_0 = Node::new();
        let node_1 = Node::new();
        assert_eq!(node_0.id, 2);
        assert_eq!(node_1.id, 3);

        let root: &mut Node = tree.get_root().unwrap();
        assert_eq!(root.id, 1);
        assert_eq!(root.get_children_len(), 0);
        root.append(node_0);
        assert_eq!(root.get_children_len(), 1);
        root.append(node_1);
        assert_eq!(root.get_children_len(), 2);

        let node_2 = Node::new();
        assert_eq!(node_2.id, 4);

        let root: &mut Node = tree.get_root().unwrap();
        root.append(node_2);
        assert_eq!(root.get_children_len(), 3);

        let root_child_1 = root.get_child_by_index(1).unwrap();
        assert_eq!(root_child_1.id, 3);
        let id = root_child_1.id;
        root.remove_by_id(id);
        root.update_value(8);
        assert_eq!(root.get_children_len(), 2);
        assert_eq!(root.get_value(), 8);

        root.remove_by_id(5);
        root.remove_by_id(5);
        root.remove_by_id(5);
        root.update_value(10);
        assert_eq!(root.get_children_len(), 2);
        assert_eq!(root.get_value(), 10);

        let boxed_node_4 = Box::new(Node::new());
        assert_eq!(boxed_node_4.id, 5);
        root.append_boxed(boxed_node_4);
        assert_eq!(root.get_children_len(), 3);
    }
}