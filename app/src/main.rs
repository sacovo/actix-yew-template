use anyhow::Result;
use my_web_app::MyTestStruct;
use yew::{
    format::{Json, Nothing, Text},
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
};

#[derive(Debug)]
enum Msg {
    AddOne,
    AddTwo,
    SetText(Option<String>),
    Fetch,
    FetchStruct,
    SetStruct(Option<MyTestStruct>),
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    text: Option<String>,
    task: Option<FetchTask>,
    obj: Option<MyTestStruct>,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            text: None,
            task: None,
            obj: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::AddTwo => {
                self.value += 2;
                true
            }
            Msg::SetText(v) => {
                self.text = v;
                true
            }
            Msg::SetStruct(data) => {
                log::trace!("Update: {:?}", data);
                self.obj = data;
                true
            }
            Msg::Fetch => {
                let request = Request::get("/hello")
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self
                    .link
                    .callback(|response: Response<Text>| Msg::SetText(response.into_body().ok()));
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.task = Some(task);
                false
            }
            Msg::FetchStruct => {
                let request = Request::get("/json-data")
                    .body(Nothing)
                    .expect("Could not build request");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<MyTestStruct>>>| {
                            log::debug!("{:?}", response);
                            let Json(data) = response.into_body();
                            Msg::SetStruct(data.ok())
                        });
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.task = Some(task);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{"+1"}</button>
                <button onclick=self.link.callback(|_| Msg::AddTwo)>{"+2"}</button>
                <button onclick=self.link.callback(|_| Msg::Fetch)>{"Fetch"}</button>
                <button onclick=self.link.callback(|_| Msg::FetchStruct)>{"Fetch"}</button>
                <p>{self.value}</p>
                <p>
                {
                    match &self.obj {
                                     Some(obj) => format!("{:?}", obj),
                                     None => format!("empty"),
                                 }}
                </p>
                <p>{self.text.as_ref().unwrap_or(&"empty".to_string())}</p>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
