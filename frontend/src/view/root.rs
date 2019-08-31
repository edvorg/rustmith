use crate::model::editor;
use crate::model::game;
use crate::model::root::*;
use crate::model::search;
use crate::registry::Registry;
use yew::prelude::*;

impl Renderable<Registry, RootModel> for RootModel {
    fn view(&self) -> Html<Registry, Self> {
        match &self.page {
            Page::Search { track_id } => {
                html! { <search::SearchModel: onsignal=RootMessage::SearchSignal, trackname=track_id, /> }
            }
            Page::Editor => {
                html! { <editor::EditorModel: onsignal=RootMessage::EditorSignal, /> }
            }
            Page::Game { song_id, song_url } => {
                html! { <game::GameModel: onsignal=RootMessage::GameSignal, songid=Some(song_id.clone()), songurl=Some(song_url.clone()), /> }
            }
        }
    }
}
