extern crate core;
extern crate stretch;

#[macro_use]
mod macros;
mod frame;
mod h_stack;
mod sized_box;
mod v_stack;
mod view;

use frame::{Frame, FrameWrapper};
use sized_box::SizedBox;
use std::cell::Cell;
use std::collections::VecDeque;
use std::default;
use std::rc::{Rc, Weak};
use stretch::geometry::Size;
use stretch::number::{Number, OrElse};
use stretch::result::Layout;
use stretch::style::{AlignContent, Dimension, FlexDirection, JustifyContent, Style};
use stretch::Stretch;
use stretch::{node::Node, Error};
use NodeType::{LayoutNode, RenderNode};

#[cfg(test)]
mod tests {
    use crate::{
        h_stack::HStack, v_stack::VStack, view::View, Content, CrossAlignment, MainAlignment,
        NodeType, RootView,
    };
    use std::rc::Rc;
    use std::sync::Weak;
    use stretch::{geometry::Size, number::Number, style::Dimension};

    #[test]
    fn it_works() {
        let v1 = View::default().absolute_frame(50.0, 100.0);
        let v2 = View::default().absolute_frame(100.0, 100.0);
        let v3 = View::default().absolute_frame(50.0, 50.0);
        let hc: Vec<Rc<dyn Content>> = vec![Rc::new(v1), Rc::new(v2)];
        let h_stack = HStack::new(
            MainAlignment::Center,
            CrossAlignment::Center,
            Some(10.0),
            hc,
        );
        let vc: Vec<Rc<dyn Content>> = vec![Rc::new(h_stack), Rc::new(v3)];
        let v_stack = VStack::new(MainAlignment::Leading, CrossAlignment::Center, None, vc);
        let root = RootView::new(Rc::new(v_stack));

        let mut sttch = stretch::node::Stretch::new();
        if let Ok(layouts) = root.compute_layout(
            &mut sttch,
            Size {
                width: Number::Defined(200.0),
                height: Number::Defined(200.0),
            },
        ) {
            for l in layouts {
                println!(
                    "layout x: {}, y: {}, width: {}, height: {}",
                    l.location.x, l.location.y, l.size.width, l.size.height
                );
            }
        }
        if let Some(n) = root.content.content_type().node() {
            let l = sttch.layout(n).unwrap();
            println!(
                "root layout: x: {}, y: {}, width: {}, height: {}",
                l.location.x, l.location.y, l.size.width, l.size.height
            );
        }
    }
}

pub enum Color {
    Red,
    Blue,
    Green,
    Write,
    Black,
    RGB(u32, u32, u32),
    RGBA(u32, u32, u32, f32),
}

pub enum NodeType {
    LayoutNode { n: Cell<Option<Node>> },
    RenderNode { n: Cell<Option<Node>> },
}

macros::define_enum_macro!(NodeType, LayoutNode, RenderNode);

impl NodeType {
    fn node(&self) -> Option<Node> {
        let NodeType! {n,..} = self;
        n.get()
    }
}

pub struct ContentIterator {
    pub(crate) dequeue: VecDeque<Rc<dyn Content>>,
}

impl Iterator for ContentIterator {
    type Item = Rc<dyn Content>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.dequeue.pop_front() {
            for e in n.children().iter().rev() {
                self.dequeue.push_front(e.clone());
            }
            Some(n)
        } else {
            None
        }
    }
}

pub struct RootView {
    pub content: Rc<dyn Content>,
    _prv: (),
}

impl RootView {
    fn new(content: Rc<dyn Content>) -> Self {
        let r = RootView { content, _prv: () };
        r.init();
        r
    }

    fn compute_layout(
        &self,
        engine: &mut Stretch,
        size: Size<Number>,
    ) -> Result<Vec<Layout>, Error> {
        self.content
            .layout_node(engine)
            .and_then(|n| engine.compute_layout(n, size))?;
        let mut layouts = vec![];
        for v in self.iter() {
            if let NodeType::RenderNode{ n: _n } = v.content_type() {
                if let Some(l) = v
                    .content_type()
                    .node()
                    .map(|n| engine.layout(n).ok())
                    .unwrap_or_else(|| None)
                {
                    let mut layout = l.clone();
                    let mut v = Rc::downgrade(&v);
                    while let Some(p_v) = v.upgrade() {
                        if let Some(l) = p_v
                            .content_type()
                            .node()
                            .map(|n| engine.layout(n).ok())
                            .unwrap_or_else(|| None)
                        {
                            layout.location.x += l.location.x;
                            layout.location.y += l.location.y;
                        }
                        if let Some(_v) = p_v.parent() {
                            v = _v;
                        } else {
                            break;
                        }
                    }
                    layouts.push(layout);
                }
            }
        }
        Ok(layouts)
    }
}

impl RootView {
    pub fn iter(&self) -> ContentIterator {
        let mut dequeue = VecDeque::new();
        dequeue.push_back(self.content.clone());
        ContentIterator { dequeue }
    }
    fn init(&self) {
        for e in self.iter() {
            for c in e.children() {
                c.set_parent(&e)
            }
        }
    }
}

pub trait Content {
    fn content_type(&self) -> NodeType;
    fn layout_node(&self, s: &mut Stretch) -> Result<Node, Error>;
    fn children(&self) -> Vec<Rc<dyn Content>>;
    fn parent(&self) -> Option<Weak<dyn Content>>;
    fn set_parent(&self, parent: &Rc<dyn Content>);

    fn default_frame(self) -> FrameWrapper
    where
        Self: Sized + 'static,
    {
        FrameWrapper::new(Rc::new(self))
    }
    fn frame(self, width: Option<f32>, height: Option<f32>) -> FrameWrapper
    where
        Self: Sized + 'static,
    {
        FrameWrapper::new(Rc::new(self)).option(width, height)
    }

    fn absolute_frame(self, width: f32, height: f32) -> FrameWrapper
    where
        Self: Sized + 'static,
    {
        FrameWrapper::new(Rc::new(self)).absolute(width, height)
    }
}

pub enum MainAlignment {
    Leading,
    Center,
    Trailing,
    Between,
    Around,
    Evenly,
}
impl MainAlignment {
    fn is_special_space(&self) -> bool {
        match self {
            MainAlignment::Leading | MainAlignment::Center | MainAlignment::Trailing => false,
            MainAlignment::Between | MainAlignment::Around | MainAlignment::Evenly => true,
        }
    }

    fn convert_to_justify_content(&self) -> JustifyContent {
        match self {
            MainAlignment::Leading => JustifyContent::FlexStart,
            MainAlignment::Center => JustifyContent::Center,
            MainAlignment::Trailing => JustifyContent::FlexEnd,
            MainAlignment::Between => JustifyContent::SpaceBetween,
            MainAlignment::Around => JustifyContent::SpaceAround,
            MainAlignment::Evenly => JustifyContent::SpaceEvenly,
        }
    }
}

pub enum CrossAlignment {
    Leading,
    Center,
    Trailing,
    Stretch,
    Between,
    Around,
}

impl CrossAlignment {
    fn convert_to_align_content(&self) -> AlignContent {
        match self {
            CrossAlignment::Leading => AlignContent::FlexStart,
            CrossAlignment::Center => AlignContent::Center,
            CrossAlignment::Trailing => AlignContent::FlexEnd,
            CrossAlignment::Stretch => AlignContent::Stretch,
            CrossAlignment::Between => AlignContent::SpaceBetween,
            CrossAlignment::Around => AlignContent::SpaceAround,
        }
    }
}
