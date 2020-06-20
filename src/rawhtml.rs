#![recursion_limit = "512"]

use web_sys::{console, Element, Node};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink, Html, ShouldRender};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHTMLProps {
    pub inner_html: String,
}

pub struct RawHTML {
    props: RawHTMLProps,
    node_ref: NodeRef,
}

impl Component for RawHTML {
    type Message = ();
    type Properties = RawHTMLProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        // create the parent element and store a reference to it
        html! {
            <div ref=self.node_ref.clone()/>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&self.props.inner_html);
    }
}
