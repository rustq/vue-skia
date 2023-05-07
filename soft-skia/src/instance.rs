use crate::tree::Tree;
use crate::tree::Node;
use crate::shape::Shapes;
use crate::shape::Shape;
use crate::shape::Rect;
use tiny_skia::{ColorU8, Paint, PathBuilder, Pixmap, Stroke, Transform, FillRule};

#[derive(Debug)]
pub struct Instance {
    pub tree: Tree
}

impl Instance {
    pub fn default() -> Self {
        Instance {
            tree: Tree::default(0)
        }
    }

    pub fn create_child_append_to_container(&mut self, child_id: usize, container_id: usize) {
        let child = Node::default(child_id, Shapes::RectShape(Rect::default()));
        let container = self.debug__get_tree_node_by_id(container_id).unwrap();
        container.append_node(child);
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
        Self::debug__recursive_find_child_node_by_id(root, id)
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
            Shapes::RectShape(Rect { x, y, width, height, color }) => {
                assert_eq!(x, 0);
                assert_eq!(y, 0);
            },
            _ => {
                panic!()
            }
        }

        instance.set_shape_to_child(1, Shapes::RectShape(Rect { x: 20, y: 20, width: 200, height: 200, color: ColorU8::from_rgba(0, 100, 0, 255) }));

        match instance.debug__get_tree_node_by_id(1).unwrap().shape {
            Shapes::RectShape(Rect { x, y, width, height, color }) => {
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