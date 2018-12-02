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
    GameSignal(game::RoutingMessage),
    SearchSignal(search::RoutingMessage),
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
        match msg {
            RootMessage::GameSignal(game::RoutingMessage::ExitGame) => {
                self.page = Page::Search;
                true
            },
            RootMessage::SearchSignal(search::RoutingMessage::StartGame { song_id: _ }) => {
                self.page = Page::Game;
                true
            },
        }
    }
}

impl Renderable<Registry, RootModel> for RootModel {
    fn view(&self) -> Html<Registry, Self> {
        match self.page {
            Page::Search =>
                html! { <search::SearchModel: onsignal=|m| RootMessage::SearchSignal(m), /> },
            Page::Game =>
                html! { <game::GameModel: onsignal=|m| RootMessage::GameSignal(m), /> },
        }
    }
}