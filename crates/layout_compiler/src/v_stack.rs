use crate::{
    sized_box::SizedBox, Content, CrossAlignment, MainAlignment, NodeType, NodeType::LayoutNode,
};
use std::cell::Cell;
use std::rc::{Rc, Weak};
use stretch::node::Node;
use stretch::style::{FlexDirection, Style};
use stretch::{Error, Stretch};

pub struct VStack {
    pub alignment: MainAlignment,
    pub cross: CrossAlignment,
    pub spacing: Option<f32>,
    pub content: Vec<Rc<dyn Content>>,
    parent: Cell<Option<Weak<dyn Content>>>,
    _node: Cell<Option<Node>>,
}

impl VStack {
    pub(crate) fn new(
        alignment: MainAlignment,
        cross: CrossAlignment,
        spacing: Option<f32>,
        content: Vec<Rc<dyn Content>>,
    ) -> Self {
        VStack {
            alignment,
            cross,
            spacing,
            content,
            parent: Cell::new(None),
            _node: Cell::new(None),
        }
    }
}

impl Content for VStack {
    fn content_type(&self) -> NodeType {
        LayoutNode {
            n: self._node.clone(),
        }
    }

    fn layout_node(&self, s: &mut Stretch) -> Result<Node, Error> {
        assert!(
            (self.alignment.is_special_space() && self.spacing.is_none())
                || !self.alignment.is_special_space(),
            "with special spacing rule should not to assert spacing"
        );
        let result = if self.alignment.is_special_space() {
            // 有指定间距的布局的情况下 忽略指定的spacing值
            let mut cns = vec![];
            for c in self.content.iter() {
                if let Ok(n) = c.layout_node(s) {
                    cns.push(n);
                }
            }
            s.new_node(
                Style {
                    justify_content: self.alignment.convert_to_justify_content(),
                    flex_direction: FlexDirection::Column,
                    align_content: self.cross.convert_to_align_content(),
                    ..Default::default()
                },
                cns,
            )
        } else if let Some(spacing) = self.spacing {
            // 指定了spacing的情况下 填充SizedBox
            let mut cns = vec![];
            for (i, e) in self.content.iter().enumerate() {
                if let Ok(n) = e.layout_node(s) {
                    cns.push(n);
                    if i < self.content.len() - 1 {
                        if let Ok(sized_box) = SizedBox::new(0.0, spacing).layout_node(s) {
                            cns.push(sized_box);
                        }
                    }
                }
            }
            s.new_node(
                Style {
                    justify_content: self.alignment.convert_to_justify_content(),
                    flex_direction: FlexDirection::Column,
                    align_content: self.cross.convert_to_align_content(),
                    ..Default::default()
                },
                cns,
            )
        } else {
            let mut cns = vec![];
            for c in self.content.iter() {
                if let Ok(n) = c.layout_node(s) {
                    cns.push(n);
                }
            }
            s.new_node(
                Style {
                    justify_content: self.alignment.convert_to_justify_content(),
                    flex_direction: FlexDirection::Column,
                    align_content: self.cross.convert_to_align_content(),
                    ..Default::default()
                },
                cns,
            )
        };
        if let Ok(node) = result {
            self._node.set(Some(node));
        }
        result
    }

    fn children(&self) -> Vec<Rc<dyn Content>> {
        let mut vec = vec![];
        for e in self.content.iter() {
            vec.push(e.clone());
        }
        vec
    }

    fn parent(&self) -> Option<Weak<dyn Content>> {
        self.parent.take()
    }

    fn set_parent(&self, parent: &Rc<dyn Content>) {
        self.parent.set(Some(Rc::downgrade(parent)));
    }
}
