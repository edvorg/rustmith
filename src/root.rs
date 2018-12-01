use yew::prelude::*;
use registry::Registry;
use game;

#[derive(Debug, PartialEq)]
pub enum Page {
    Logo,
    Game,
}

pub enum RootMessage {
    SwitchPage(Page),
    GameSignal,
}

pub struct RootModel {
    page: Page
}

impl Component<Registry> for RootModel {
    type Message = RootMessage;
    type Properties = ();

    fn create(_props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        env.console.log("creating root model");
        RootModel { page: Page::Logo }
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
        }
    }
}

impl RootModel {
    fn buttons_view(&self) -> Html<Registry, Self> {
        html! {
        <>
          <button onclick = |_| RootMessage::SwitchPage(Page::Game) ,> { "game" } </button>
          <button onclick = |_| RootMessage::SwitchPage(Page::Logo) ,> { "logo" } </button>
        </>
        }
    }

    fn page_view(&self) -> Html<Registry, Self> {
        match self.page {
            Page::Logo =>
                html! { <div> { "logo" } </div> },
            Page::Game =>
                html! { <div><game::GameModel: onsignal=|_| RootMessage::GameSignal, /></div> },
        }
    }
}

impl Renderable<Registry, RootModel> for RootModel {
    fn view(&self) -> Html<Registry, Self> {
        html! {
        <div>
          { self.page_view() }
          { self.buttons_view() }
        </div>
        }
    }
}