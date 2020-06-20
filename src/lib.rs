use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod editor;
pub mod header;
pub mod rawhtml;

use self::editor::Editor;
use self::header::Header;


struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }
    
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Header/>
                <Editor/>
                <button onclick=self.link.callback(|_| Msg::AddOne)>
                { "+1" }
                </button>
                <p>{ self.value }</p>
            </div>
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
