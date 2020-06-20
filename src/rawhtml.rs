#![recursion_limit = "512"]

use web_sys::{console, Node};
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink, Html, ShouldRender};
use yew::prelude::*;

const SVG: &str = r#"
<h2>Rajat Rajat Rajat</h2>
"#;

pub struct RawHtml {
    data : String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub data: String,
}

pub enum Msg {}

impl Component for RawHtml {
    type Message = Msg;
    type Properties = Props; 

    fn create(props : Self::Properties, _: ComponentLink<Self>) -> Self {
        RawHtml { 
            data : props.data
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props :Self::Properties) -> ShouldRender {
        self.data = props.data;
        true
    }

    fn view(&self) -> Html {
        let vg = {
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("main")
                .unwrap();

            // let div = web_sys::window()
            //     .unwrap()
            //     .document()
            //     .unwrap()
            //     .create_element("div")
            //     .unwrap();
            div.set_inner_html(&self.data);
            // console::log_1(&div);
            div
        };
        eprintln!("js_svg: {:?}", vg);
        let node = Node::from(vg);
        let vnode = VNode::VRef(node);
        eprintln!("svg: {:?}", vnode);
        vnode
    }

}
