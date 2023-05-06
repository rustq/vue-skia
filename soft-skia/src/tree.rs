use std::slice::Iter;
use std::slice::IterMut;
use tiny_skia::{ColorU8};
use crate::shape::Shape;

#[derive(Debug)]
pub struct Tree {
    root: Box<Node>
}

#[derive(Debug)]
pub struct Node {
    id: usize,
    shape: Shape,
    children: Vec<Box<Node>>
}

impl Tree {
    pub fn new(id: usize) -> Self {
        Tree {
            root: Box::new(Node { id, shape: Shape::Rect { x: 0, y: 0, width: 0, height: 0, color: ColorU8::from_rgba(0, 0, 0, 255) }, children: Vec::new() })
        }
    }

    pub fn get_root(&mut self) -> &mut Node {
        self.root.as_mut()
    }
}

impl Node {
    pub fn append_node(&mut self, node: Node) {
        self.children.push(Box::new(node));
    }

    pub fn append_boxed_node(&mut self, boxed_node: Box<Node>) {
        self.children.push(boxed_node);
    }

    pub fn get_children_len(&self) -> usize {
        self.children.len()
    }

    pub fn get_mut_child_by_index(&mut self, index: usize) -> Option<&mut Node> {
        Some(self.children[index].as_mut())
    }

    pub fn get_child_by_index(&mut self, index: usize) -> Option<&Node> {
        Some(self.children[index].as_ref())
    }

    pub fn children_iter(&mut self) -> Iter<Box<Node>> {
        self.children.iter()
    }

    pub fn children_iter_mut(&mut self) -> IterMut<Box<Node>> {
        self.children.iter_mut()
    }

    pub fn remove_child_by_index(&mut self, index: usize) {
        self.children.remove(index);
    }

    pub fn remove_child_by_id(&mut self, node_id: usize) {
        self.children.retain(|t| t.id != node_id);
    }
}

#[cfg(test)]
mod test {
    use super::Tree;
    use super::Node;
    use super::Shape;
    use super::ColorU8;

    #[test]
    fn test_tree() {
        let mut tree = Tree::new(0);
        let mut root = tree.get_root();

        assert_eq!(root.id, 0);
        assert_eq!(root.get_children_len(), 0);

        root.id = 10086;
        assert_eq!(root.id, 10086);

        match root.shape {
            Shape::Rect { x, y, width, height, color } => {
                assert_eq!(width, 0);
                assert_eq!(height, 0);
            },
            _ => {
                panic!()
            }
        }

        root.shape = Shape::Rect { x: 0, y: 0, width: 400, height: 400, color: ColorU8::from_rgba(0, 0, 0, 255) };
        match root.shape {
            Shape::Rect { x, y, width, height, color } => {
                assert_eq!(width, 400);
                assert_eq!(height, 400);
            },
            _ => {
                panic!()
            }
        }
    }

    #[test]
    fn test_node() {
        let mut node = Node { id: 0, shape: Shape::Rect { x: 0, y: 0, width: 400, height: 400, color: ColorU8::from_rgba(0, 0, 0, 255) }, children: Vec::new() };

        assert_eq!(node.id, 0);
        assert_eq!(node.get_children_len(), 0);

        node.append_node(Node { id: 1, shape: Shape::Circle { c: 100, r: 50, color: ColorU8::from_rgba(0, 0, 0, 255) }, children: Vec::new() });
        assert_eq!(node.get_children_len(), 1);

        node.append_node(Node { id: 2, shape: Shape::Circle { c: 100, r: 50, color: ColorU8::from_rgba(0, 0, 0, 255) }, children: Vec::new() });
        assert_eq!(node.get_children_len(), 2);

        node.append_boxed_node(Box::new(Node { id: 3, shape: Shape::Circle { c: 100, r: 50, color: ColorU8::from_rgba(0, 0, 0, 255) }, children: Vec::new() }));
        node.append_boxed_node(Box::new(Node { id: 4, shape: Shape::Circle { c: 100, r: 50, color: ColorU8::from_rgba(0, 0, 0, 255) }, children: Vec::new() }));
        assert_eq!(node.get_children_len(), 4);

        let child_index_0 = node.get_child_by_index(0).unwrap();
        assert_eq!(child_index_0.id, 1);

        let mut child_index_1 = node.get_mut_child_by_index(1).unwrap();
        assert_eq!(child_index_1.id, 2);
        child_index_1.id = 100;
        assert_eq!(child_index_1.id, 100);

        node.remove_child_by_index(1);
        assert_eq!(node.get_children_len(), 3);
        assert_eq!(node.children[0].id, 1);
        assert_eq!(node.children[1].id, 3);
        assert_eq!(node.children[2].id, 4);

        node.remove_child_by_id(3);
        assert_eq!(node.get_children_len(), 2);
        assert_eq!(node.children[0].id, 1);
        assert_eq!(node.children[1].id, 4);

        for item in node.children_iter_mut() {
            match item.shape {
                Shape::Circle { ref mut c, r, color } => {
                    assert_eq!(*c, 100);
                    *c = 200;
                    assert_eq!(*c, 200);
                },
                _ => panic!(),
            }
        }

        for item in node.children_iter() {
            match item.shape {
                Shape::Circle { c, r, color } => {
                    assert_eq!(c, 200);
                },
                _ => panic!(),
            }
        }
    }
}