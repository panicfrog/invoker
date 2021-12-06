use crate::{Color, Content, NodeType, NodeType::RenderNode, Attribute};
use std::cell::Cell;
use std::default;
use std::rc::{Rc, Weak};
use stretch::node::Node;
use stretch::style::Style;
use stretch::{Error, Stretch};

pub struct View {
    pub background_color: Color,
    pub child: Option<Rc<dyn Content>>,
    pub parent: Cell<Option<Weak<dyn Content>>>,
    _node: Cell<Option<Node>>,
}

impl default::Default for View {
    fn default() -> Self {
        View {
            background_color: Color::Write,
            child: None,
            parent: Cell::new(None),
            _node: Cell::new(None),
        }
    }
}

impl Content for View {
    fn content_type(&self) -> NodeType {
        RenderNode {
            n: self._node.clone(),
        }
    }

    fn layout_node(&self, s: &mut Stretch) -> Result<Node, Error> {
        let result = if let Some(child) = &self.child {
            if let Ok(cn) = child.layout_node(s) {
                s.new_node(
                    Style {
                        flex_grow: 1.0,
                        ..Default::default()
                    },
                    vec![cn],
                )
            } else {
                s.new_node(
                    Style {
                        flex_grow: 1.0,
                        ..Default::default()
                    },
                    vec![],
                )
            }
        } else {
            s.new_node(
                Style {
                    flex_grow: 1.0,
                    ..Default::default()
                },
                vec![],
            )
        };
        if let Ok(node) = result {
            self._node.set(Some(node));
        }
        result
    }

    fn children(&self) -> Vec<Rc<dyn Content>> {
        if let Some(child) = &self.child {
            vec![child.clone()]
        } else {
            vec![]
        }
    }

    fn parent(&self) -> Option<Weak<dyn Content>> {
        self.parent.take()
    }

    fn set_parent(&self, parent: &Rc<dyn Content>) {
        self.parent.set(Some(Rc::downgrade(parent)));
    }

    fn attribute(&self) -> Attribute {
        Attribute::default()
    }
}
