use crate::{Content, NodeType, NodeType::LayoutNode};
use std::cell::Cell;
use std::rc::{Rc, Weak};
use stretch::geometry::Size;
use stretch::node::Node;
use stretch::style::{Dimension, Style};
use stretch::{Error, Stretch};

pub struct Frame {
    pub width: Option<f32>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub height: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    _prv: (),
}

impl Frame {
    pub(crate) fn width(mut self, value: Option<f32>) -> Self {
        self.width = value;
        self
    }
    fn min_width(mut self, value: Option<f32>) -> Self {
        self.min_width = value;
        self
    }
    fn max_width(mut self, value: Option<f32>) -> Self {
        self.max_width = value;
        self
    }
    pub(crate) fn height(mut self, value: Option<f32>) -> Self {
        self.height = value;
        self
    }
    fn min_height(mut self, value: Option<f32>) -> Self {
        self.min_height = value;
        self
    }
    fn max_height(mut self, value: Option<f32>) -> Self {
        self.max_height = value;
        self
    }

    fn size(&self) -> Size<Dimension> {
        let width = convert_options(self.width);
        let height = convert_options(self.height);
        Size { width, height }
    }

    fn min_size(&self) -> Size<Dimension> {
        let width = convert_options(self.min_width);
        let height = convert_options(self.min_height);
        Size { width, height }
    }

    fn max_size(&self) -> Size<Dimension> {
        let width = convert_options(self.max_width);
        let height = convert_options(self.max_height);
        Size { width, height }
    }
}

fn convert_options(value: Option<f32>) -> Dimension {
    if let Some(v) = value {
        Dimension::Points(v)
    } else {
        Dimension::Undefined
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            width: None,
            min_width: None,
            max_width: None,
            height: None,
            min_height: None,
            max_height: None,
            _prv: (),
        }
    }
}

pub struct FrameWrapper {
    pub frame: Frame,
    pub content: Rc<dyn Content>,
    pub parent: Cell<Option<Weak<dyn Content>>>,
    _node: Cell<Option<Node>>,
}

impl Content for FrameWrapper {
    fn content_type(&self) -> NodeType {
        LayoutNode {
            n: self._node.clone(),
        }
    }

    fn layout_node(&self, s: &mut Stretch) -> Result<Node, Error> {
        let size = self.frame.size();
        let min_size = self.frame.min_size();
        let max_size = self.frame.max_size();
        let result = if let Ok(cn) = self.content.layout_node(s) {
            s.new_node(
                Style {
                    size,
                    min_size,
                    max_size,
                    ..Default::default()
                },
                vec![cn],
            )
        } else {
            s.new_node(
                Style {
                    size,
                    min_size,
                    max_size,
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
        vec![self.content.clone()]
    }

    fn parent(&self) -> Option<Weak<dyn Content>> {
        self.parent.take()
    }

    fn set_parent(&self, parent: &Rc<dyn Content>) {
        self.parent.set(Some(Rc::downgrade(parent)));
    }
}

impl FrameWrapper {
    pub fn new(content: Rc<dyn Content>) -> Self {
        FrameWrapper {
            frame: Default::default(),
            content,
            parent: Cell::new(None),
            _node: Cell::new(None),
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.frame.width = Some(width);
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.frame.height = Some(height);
        self
    }
    pub fn absolute(mut self, width: f32, height: f32) -> Self {
        self.frame = Frame::default().width(Some(width)).height(Some(height));
        self
    }
    pub fn option(mut self, width: Option<f32>, height: Option<f32>) -> Self {
        self.frame = Frame::default().width(width).height(height);
        self
    }
}
