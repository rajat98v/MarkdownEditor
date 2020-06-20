use yew::events::KeyboardEvent;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::utils::NodeSeq;
use yew::virtual_dom::VChild;
use yew::virtual_dom::VComp;

use crate::rawhtml::RawHTML;

pub struct Editor {
    link: ComponentLink<Self>,
    onsignal: Callback<()>,
    text: Vec<String>,
    last: usize,
    print: String,
}

pub enum Msg {
    ChildClicked,
    KeyPressed(KeyboardEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub onsignal: Callback<()>,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Editor {
            print: String::from("<h1>It is successful</h1>"),
            link,
            onsignal: props.onsignal,
            // text: Vec::new(),
            text: vec!["Edit Here...".to_string(); 1],
            last: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChildClicked => {
                // self.onsignal.emit(());
            }
            Msg::KeyPressed(e) => {
                match e.char_code() {
                    13 => {
                        self.text.push("".to_string());
                    }
                    _ => {
                        if let Some(last) = self.text.last_mut() {
                            last.push(e.char_code() as u8 as char);
                        }
                    }
                }
                // ConsoleService::new().log(&format!("updateate: {:?}", self.text));
                // ConsoleService::new().log(&format!("updateate: {:?}", e.char_code()));
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div tabindex="1"  onkeypress=self.link.callback(|e : KeyboardEvent | Msg::KeyPressed(e))>
                <div id="main" class="editor" >
                    <h4>{"Edit You Notes Here"}</h4>
                </div>
                // self.text is vector of String
                <>{self.text.iter().map(|line| self.view_line(line)).collect::<Html>()}</>
                // {self.view_text()}
            </div>
        }
    }
}

impl Editor {
    fn view_line(&self, text: &String) -> Html {
        ConsoleService::new().log(&format!("updateate: {}", text));
        html! {
            <div>
                <RawHTML inner_html=text/>
            </div>
        }
    }
    fn view_text(&self) -> Html {
        html! {
            <div>
            { for self.text.iter().map(|line| {self.view_line(line)}) }
            </div>
        }
    }
}
