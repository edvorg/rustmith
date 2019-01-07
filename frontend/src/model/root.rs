use crate::model::editor;
use crate::model::game;
use crate::registry::Registry;
use crate::model::search;
use yew::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Page {
    Search { track_id: Option<String> },
    Editor,
    Game { song_id: String, song_url: String },
}

pub enum RootMessage {
    GameSignal(game::RoutingMessage),
    SearchSignal(search::RoutingMessage),
    EditorSignal(editor::RoutingMessage),
}

pub struct RootModel {
    pub page: Page,
}

impl Component<Registry> for RootModel {
    type Message = RootMessage;
    type Properties = ();

    fn create(_props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        env.console.log("creating root model");
        RootModel { page: Page::Search { track_id: None } }
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<Registry, Self>) -> bool {
        match msg {
            RootMessage::GameSignal(game::RoutingMessage::ExitGame) => {
                self.page = Page::Search { track_id: None };
                true
            }
            RootMessage::SearchSignal(search::RoutingMessage::StartGame { song_id, song_url }) => {
                self.page = Page::Game { song_id, song_url };
                true
            }
            RootMessage::SearchSignal(search::RoutingMessage::NewSong) => {
                self.page = Page::Editor;
                true
            }
            RootMessage::EditorSignal(editor::RoutingMessage::Exit) => {
                self.page = Page::Search { track_id: None };
                true
            }
            RootMessage::EditorSignal(editor::RoutingMessage::ExitAndShowTrack(id)) => {
                self.page = Page::Search { track_id: Some(id) };
                true
            }
        }
    }
}
