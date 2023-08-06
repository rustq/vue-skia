use crate::provider::Providers;
use crate::shape::Circle;
use crate::shape::Rect;
use crate::shape::Shape;
use crate::shape::DrawContext;
use crate::shape::Shapes;
use std::slice::Iter;
use std::slice::IterMut;
use tiny_skia::ColorU8;
use tiny_skia::Pixmap;

#[derive(Debug)]
pub struct Tree {
    root: Box<Node>,
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub shape: Shapes,
    pub provider: Option<Providers>,
    pub children: Vec<Box<Node>>,
}

impl Tree {
    pub fn default(id: usize) -> Self {
        Tree {
            root: Box::new(Node {
                id,
                shape: Shapes::R(Rect::default()),
                children: Vec::new(),
                provider: None,
            }),
        }
    }

    pub fn get_root(&mut self) -> &mut Node {
        self.root.as_mut()
    }
}

impl Node {
    pub fn default(id: usize, shape: Shapes) -> Self {
        Node {
            id,
            shape,
            children: Vec::new(),
            provider: None,
        }
    }

    pub fn draw(&self, pixmap: &mut Pixmap, context: Option<&DrawContext>) {
        self.shape
            .draw(pixmap, context.unwrap_or(&DrawContext::default()));
    }

    pub fn append_node(&mut self, node: Node) {
        self.append_boxed_node(Box::new(node));
    }

    pub fn insert_node_before_id(&mut self, before_id: usize, node: Node) {
        self.insert_boxed_node_before_id(before_id, Box::new(node));
    }

    pub fn insert_boxed_node_before_id(&mut self, before_id: usize, boxed_node: Box<Node>) {
        if let Some(before_index) = self.children.iter().position(|t| t.id == before_id) {
            self.children.insert(before_index, boxed_node);
        } else {
            self.append_boxed_node(boxed_node)
        }
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
    use crate::shape::PaintStyle;

    use super::Circle;
    use super::ColorU8;
    use super::Node;
    use super::Rect;
    use super::Shapes;
    use super::Tree;

    #[test]
    fn test_tree() {
        let mut tree = Tree::default(0);
        let mut root = tree.get_root();

        assert_eq!(root.id, 0);
        assert_eq!(root.get_children_len(), 0);

        root.id = 10086;
        assert_eq!(root.id, 10086);

        match root.shape {
            Shapes::R(Rect {
                x,
                y,
                width,
                height,
                color,
                style,
            }) => {
                assert_eq!(width, 0);
                assert_eq!(height, 0);
            }
            _ => {
                panic!()
            }
        }

        root.shape = Shapes::R(Rect {
            x: 0,
            y: 0,
            width: 400,
            height: 400,
            color: None,
            style: None,
        });
        match root.shape {
            Shapes::R(Rect {
                x,
                y,
                width,
                height,
                color,
                style: None,
            }) => {
                assert_eq!(width, 400);
                assert_eq!(height, 400);
            }
            _ => {
                panic!()
            }
        }
    }

    #[test]
    fn test_node() {
        let mut node = Node {
            id: 0,
            shape: Shapes::R(Rect {
                x: 0,
                y: 0,
                width: 400,
                height: 400,
                color: None,
                style: None,
            }),
            children: Vec::new(),
            provider: None,
        };

        assert_eq!(node.id, 0);
        assert_eq!(node.get_children_len(), 0);

        node.append_node(Node {
            id: 1,
            shape: Shapes::C(Circle {
                cx: 100,
                cy: 100,
                r: 50,
                color: None,
                style: None,
            }),
            children: Vec::new(),
            provider: None,
        });
        assert_eq!(node.get_children_len(), 1);

        node.append_node(Node {
            id: 2,
            shape: Shapes::C(Circle {
                cx: 100,
                cy: 100,
                r: 50,
                color: None,
                style: None,
            }),
            children: Vec::new(),
            provider: None,
        });
        assert_eq!(node.get_children_len(), 2);

        node.append_boxed_node(Box::new(Node {
            id: 3,
            shape: Shapes::C(Circle {
                cx: 100,
                cy: 100,
                r: 50,
                color: None,
                style: None,
            }),
            children: Vec::new(),
            provider: None,
        }));
        node.append_boxed_node(Box::new(Node {
            id: 4,
            shape: Shapes::C(Circle {
                cx: 100,
                cy: 100,
                r: 50,
                color: None,
                style: None,
            }),
            children: Vec::new(),
            provider: None,
        }));
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

        node.insert_node_before_id(
            4,
            Node {
                id: 200,
                shape: Shapes::C(Circle {
                    cx: 100,
                    cy: 100,
                    r: 50,
                    color: None,
                    style: None,
                }),
                children: Vec::new(),
                provider: None,
            },
        );
        assert_eq!(node.get_children_len(), 3);
        assert_eq!(node.children[0].id, 1);
        assert_eq!(node.children[1].id, 200);
        assert_eq!(node.children[2].id, 4);

        node.insert_node_before_id(
            4,
            Node {
                id: 300,
                shape: Shapes::C(Circle {
                    cx: 100,
                    cy: 100,
                    r: 50,
                    color: None,
                    style: None,
                }),
                children: Vec::new(),
                provider: None,
            },
        );
        assert_eq!(node.get_children_len(), 4);
        assert_eq!(node.children[0].id, 1);
        assert_eq!(node.children[1].id, 200);
        assert_eq!(node.children[2].id, 300);
        assert_eq!(node.children[3].id, 4);

        for item in node.children_iter_mut() {
            match item.shape {
                Shapes::C(Circle {
                    ref mut cx,
                    cy,
                    r,
                    color,
                    style,
                }) => {
                    assert_eq!(*cx, 100);
                    *cx = 200;
                    assert_eq!(*cx, 200);
                }
                _ => panic!(),
            }
        }

        for item in node.children_iter() {
            match item.shape {
                Shapes::C(Circle {
                    cx,
                    cy,
                    r,
                    color,
                    style,
                }) => {
                    assert_eq!(cx, 200);
                }
                _ => panic!(),
            }
        }
    }
}
