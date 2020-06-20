use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>,
    onsignal: Callback<()>
}

pub enum Msg {
    ChildClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub onsignal: Callback<()>
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Header {
            link,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChildClicked => {
                // self.onsignal.emit(());
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
            <div class="header">
                <element> {"Mark"}</element> <element class="cursor" ></element> <element>{"down Notes"}</element>
            </div>
        }
    }
}
