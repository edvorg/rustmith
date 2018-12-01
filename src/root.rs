use yew::prelude::*;
use registry::Registry;
use game;
use search;

#[derive(Debug, PartialEq)]
pub enum Page {
    Search,
    Game,
}

pub enum RootMessage {
    SwitchPage(Page),
    GameSignal,
    LogoSignal,
}

pub struct RootModel {
    page: Page
}

impl Component<Registry> for RootModel {
    type Message = RootMessage;
    type Properties = ();

    fn create(_props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        env.console.log("creating root model");
        RootModel { page: Page::Search }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        env.console.log("updating root model");
        match msg {
            RootMessage::SwitchPage(page) => {
                if page == self.page {
                    false
                } else {
                    self.page = page;
                    true
                }
            },
            RootMessage::GameSignal => {
                env.console.log("received game signal");
                false
            },
            RootMessage::LogoSignal => {
                env.console.log("received logo signal");
                false
            },
        }
    }
}

impl RootModel {
    fn buttons_view(&self) -> Html<Registry, Self> {
        html! {
        <>
          <button onclick = |_| RootMessage::SwitchPage(Page::Game) ,> { "game" } </button>
          <button onclick = |_| RootMessage::SwitchPage(Page::Search) ,> { "search" } </button>
        </>
        }
    }

    fn page_view(&self) -> Html<Registry, Self> {
        match self.page {
            Page::Search =>
                html! { <search::SearchModel: onsignal=|_| RootMessage::LogoSignal, /> },
            Page::Game =>
                html! { <game::GameModel: onsignal=|_| RootMessage::GameSignal, /> },
        }
    }
}

impl Renderable<Registry, RootModel> for RootModel {
    fn view(&self) -> Html<Registry, Self> {
        html! {
        <>
          { self.page_view() }
          { self.buttons_view() }
        </>
        }
    }
}