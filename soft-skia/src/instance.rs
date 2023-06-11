use std::collections::HashMap;

use crate::tree::Tree;
use crate::tree::Node;
use crate::shape::Shapes;
use crate::shape::Shape;
use crate::shape::Rect;
use tiny_skia::{ColorU8, Paint, PathBuilder, Pixmap, Stroke, Transform, FillRule};

#[derive(Debug)]
pub struct Instance {
    pub tree: Tree,
    pub node_ptr_map: HashMap<usize, *mut Node>
}

impl Instance {
    pub fn new(id: usize) -> Self {
        Instance {
            tree: Tree::default(id),
            node_ptr_map: HashMap::new()
        }
    }

    #[deprecated(since="0.0.2", note="please use `new` instead")]
    pub fn default() -> Self {
        Instance {
            tree: Tree::default(0),
            node_ptr_map: HashMap::new()
        }
    }

    pub fn create_child_append_to_container(&mut self, child_id: usize, container_id: usize) {
        let mut child = Box::new(Node::default(child_id, Shapes::R(Rect::default())));
        let container = self.debug__get_tree_node_by_id(container_id).unwrap();
        let raw_child: *mut _ = &mut *child;
        container.append_boxed_node(child);
        self.node_ptr_map.insert(child_id, raw_child);
    }

    pub fn create_child_insert_before_element_of_container(&mut self, child_id: usize, insert_before_id: usize, container_id: usize) {
        let mut child = Box::new(Node::default(child_id, Shapes::R(Rect::default())));
        let container = self.debug__get_tree_node_by_id(container_id).unwrap();
        let raw_child: *mut _ = &mut *child;
        container.insert_boxed_node_before_id(insert_before_id, child);
        self.node_ptr_map.insert(child_id, raw_child);
    }

    pub fn set_shape_to_child(&mut self, child_id: usize, shape: Shapes) {
        let mut child = self.debug__get_tree_node_by_id(child_id).unwrap();
        child.shape = shape;
    }

    pub fn remove_child_from_container(&mut self, child_id: usize, container_id: usize) {
        let container = self.debug__get_tree_node_by_id(container_id).unwrap();
        container.remove_child_by_id(child_id);
    }

    ///
    /// Debug
    ///
    pub fn debug__get_tree_node_by_id(&mut self, id: usize) -> Option<&mut Node> {
        let root = self.tree.get_root();

        if id == root.id {
            Some(root)
        } else {
            let raw_node = self.node_ptr_map.get(&id);
            match raw_node {
                Some(row_node) => {
                    unsafe {
                        Some(&mut **row_node)
                    }
                },
                _ => None
            }
        }
        // Self::debug__recursive_find_child_node_by_id(root, id)
    }

    ///
    /// Debug
    ///
    fn debug__recursive_find_child_node_by_id(parent: &mut Node, child_node_id: usize) -> Option<&mut Node> {
        if parent.id == child_node_id {
            return Some(parent)
        }
        if parent.children.len() == 0 {
            return None;
        }
        for item in parent.children.iter_mut() {
            match Self::debug__recursive_find_child_node_by_id(item, child_node_id) {
                Some(target) => return Some(target),
                None => { continue }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::shape::Rect;
    use crate::shape::Circle;
    use crate::shape::Shape;
    use crate::shape::Shapes;
    use crate::shape::PaintStyle;
    use crate::tree::Node;
    use super::Instance;
    use super::{ColorU8, Paint, PathBuilder, Pixmap, Stroke, Transform, FillRule};

    #[test]
    fn test_slot_map() {
        //
    }

    #[test]
    fn test_get_tree_node_by_id() {
        let mut instance = Instance::default();

        let target_0 = instance.debug__get_tree_node_by_id(0);
        assert_eq!(target_0.is_none(), false);
        assert_eq!(target_0.unwrap().id, 0);

        let target_3 = instance.debug__get_tree_node_by_id(3);
        assert_eq!(target_3.is_none(), true);
    }

    #[test]
    fn test_append() {
        let mut instance = Instance::default();
        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 0);

        instance.create_child_append_to_container(1, 0);
        instance.create_child_append_to_container(2, 0);
        instance.create_child_append_to_container(3, 0);

        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 3);
    }

    #[test]
    fn test_set_shape() {
        let mut instance = Instance::default();
        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 0);

        instance.create_child_append_to_container(1, 0);
        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 1);

        match instance.debug__get_tree_node_by_id(1).unwrap().shape {
            Shapes::R(Rect { x, y, width, height, color, style }) => {
                assert_eq!(x, 0);
                assert_eq!(y, 0);
            },
            _ => {
                panic!()
            }
        }

        instance.set_shape_to_child(1, Shapes::R(Rect { x: 20, y: 20, width: 200, height: 200, color: ColorU8::from_rgba(0, 100, 0, 255), style: PaintStyle::Fill }));

        match instance.debug__get_tree_node_by_id(1).unwrap().shape {
            Shapes::R(Rect { x, y, width, height, color, style }) => {
                assert_eq!(x, 20);
                assert_eq!(y, 20);
                assert_eq!(width, 200);
                assert_eq!(height, 200);
                assert_eq!(color.green(), 100);
            },
            _ => {
                panic!()
            }
        }
    }

    #[test]
    fn test_remove() {
        let mut instance = Instance::default();
        instance.create_child_append_to_container(1, 0);
        instance.create_child_append_to_container(2, 0);
        instance.create_child_append_to_container(3, 0);

        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 3);

        instance.remove_child_from_container(2, 0);
        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 2);

        instance.remove_child_from_container(3, 0);
        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 1);

        instance.remove_child_from_container(1, 0);
        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 0);

        instance.remove_child_from_container(1, 0);
        instance.remove_child_from_container(1, 0);
        instance.remove_child_from_container(1, 0);
        instance.remove_child_from_container(100, 0);
        instance.remove_child_from_container(100, 0);
        instance.remove_child_from_container(100, 0);
        assert_eq!(instance.debug__get_tree_node_by_id(0).unwrap().get_children_len(), 0);
    }
}