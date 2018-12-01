use yew::prelude::*;
use registry::Registry;
use game;

#[derive(Debug, PartialEq)]
pub enum Page {
    Logo,
    Game,
}

pub enum PagingMessage {
    SwitchPage(Page),
    GameSignal,
}

pub struct PagingModel {
    page: Page
}

impl Component<Registry> for PagingModel {
    type Message = PagingMessage;
    type Properties = ();

    fn create(_props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        env.console.log("creating paging model");
        PagingModel { page: Page::Logo }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        env.console.log("updating paging model");
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
                env.console.log("received game signal");
                false
            },
        }
    }
}

impl PagingModel {
    fn buttons_view(&self) -> Html<Registry, Self> {
        html! {
        <>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Game) ,> { "game" } </button>
          <button onclick = |_| PagingMessage::SwitchPage(Page::Logo) ,> { "logo" } </button>
        </>
        }
    }

    fn page_view(&self) -> Html<Registry, Self> {
        match self.page {
            Page::Logo =>
                html! { <div> { "logo" } </div> },
            Page::Game =>
                html! { <div><game::GameModel: onsignal=|_| PagingMessage::GameSignal, /></div> },
        }
    }
}

impl Renderable<Registry, PagingModel> for PagingModel {
    fn view(&self) -> Html<Registry, Self> {
        html! {
        <div>
          { self.page_view() }
          { self.buttons_view() }
        </div>
        }
    }
}