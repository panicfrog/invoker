use crate::{Content, NodeType, NodeType::LayoutNode};
use std::cell::Cell;
use std::rc::{Rc, Weak};
use stretch::geometry::Size;
use stretch::node::Node;
use stretch::style::{Dimension, Style};
use stretch::{Error, Stretch};

pub struct SizedBox {
    pub width: f32,
    pub height: f32,
    parent: Cell<Option<Weak<dyn Content>>>,
    _node: Cell<Option<Node>>,
}

impl SizedBox {
    pub fn new(width: f32, height: f32) -> Self {
        SizedBox {
            width,
            height,
            parent: Cell::new(None),
            _node: Cell::new(None),
        }
    }
}

impl Content for SizedBox {
    fn content_type(&self) -> NodeType {
        LayoutNode {
            n: self._node.clone(),
        }
    }

    fn layout_node(&self, s: &mut Stretch) -> Result<Node, Error> {
        let result = s.new_node(
            Style {
                size: Size {
                    width: Dimension::Points(self.width),
                    height: Dimension::Points(self.height),
                },
                ..Default::default()
            },
            vec![],
        );
        if let Ok(node) = result {
            self._node.set(Some(node))
        }

        result
    }

    fn children(&self) -> Vec<Rc<dyn Content>> {
        vec![]
    }

    fn parent(&self) -> Option<Weak<dyn Content>> {
        self.parent.take()
    }

    fn set_parent(&self, parent: &Rc<dyn Content>) {
        self.parent.set(Some(Rc::downgrade(parent)))
    }
}
