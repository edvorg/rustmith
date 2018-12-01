#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate yew;

mod context;
mod game;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use context::Context;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum Page {
    Logo,
    Game,
}

enum PagingMessage {
    SwitchPage(Page),
    GameSignal,
}

struct PagingModel {
    page: Page
}

impl Component<Context> for PagingModel {
    type Message = PagingMessage;
    type Properties = ();

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.log("creating paging model");
        PagingModel { page: Page::Logo }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> bool {
        context.console.log("updating paging model");
        match msg {
            PagingMessage::SwitchPage(page) => {
                if page == self.page {
                    false
                } else {
                    self.page = page;
                    true
                }
            },
            PagingMessage::GameSignal => {
                context.console.log("received game signal");
                false
            },
        }
    }
}

impl PagingModel {
    fn buttons_view(&self) -> Html<Context, Self> {
        html! {
        <>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Game) ,> { "game" } </button>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Logo) ,> { "logo" } </button>
        </>
        }
    }

    fn page_view(&self) -> Html<Context, Self> {
        match self.page {
            Page::Logo =>
                html! { <div> { "logo" } </div> },
            Page::Game =>
                html! { <div><game::GameModel: onsignal=|_| PagingMessage::GameSignal, /></div> },
        }
    }
}

impl Renderable<Context, PagingModel> for PagingModel {
    fn view(&self) -> Html<Context, Self> {
        html! {
        <div>
          { self.page_view() }
          { self.buttons_view() }
        </div>
        }
    }
}

fn main() {
    yew::initialize();
    let console = ConsoleService::new();
    let timeout = TimeoutService::new();
    let context = Context { console, timeout };
    let app = App::<Context, PagingModel>::new(context);
    app.mount_to_body();
    yew::run_loop();
}
