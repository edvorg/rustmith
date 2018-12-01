#[macro_use]
extern crate yew;

use yew::prelude::*;

type Context = ();

#[derive(Debug, PartialEq)]
enum Page {
    Logo,
    Search,
    Game,
    Result,
}

enum PagingMessage {
    SwitchPage(Page)
}

struct PagingModel {
    page: Page
}

impl Component<Context> for PagingModel {
    type Message = PagingMessage;
    type Properties = ();

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        PagingModel { page: Page::Logo }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> bool {
        match msg {
            PagingMessage::SwitchPage(page) =>
                if page == self.page {
                    false
                } else {
                    self.page = page;
                    true
                },
            _ =>
                false,
        }
    }
}

struct LogoModel {
}

impl Component<Context> for LogoModel {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, context: &mut Env<(), Self>) -> Self {
        unimplemented!()
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<(), Self>) -> bool {
        unimplemented!()
    }
}

struct SearchModel {
}

struct GameModel {
}

struct ResultModel {
}

impl Renderable<Context, PagingModel> for PagingModel {
    fn view(&self) -> Html<Context, Self> {
        html! {
        <div>
          <div>{ format!("hello world, page is {:?}", &self.page) }</div>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Game) ,> { "game" } </button>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Logo) ,> { "logo" } </button>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Result) ,> { "result" } </button>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Search) ,> { "search" } </button>
        </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Context, PagingModel>::new(()).mount_to_body();
    yew::run_loop();
}
