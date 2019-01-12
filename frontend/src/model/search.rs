use crate::registry::Registry;
use crate::services::track::TrackService;
use yew::prelude::*;
use rustmith_common::track::SearchResponse;
use yew::services::fetch::FetchTask;

#[derive(Debug)]
pub enum RoutingMessage {
    /// switch to game screen and load song with id song_id
    StartGame {
        song_id: String,
        song_url: String,
    },
    NewSong,
}

#[derive(Debug)]
pub enum SearchMessage {
    UpdateSearchString(String),
    Search,
    UnknownKey,
    ResultsReceived(SearchResponse),
    LoadMore { term: String, continuation_token: String },
    Route(RoutingMessage),
}

pub struct SearchModel {
    pub search_str: String,
    pub search_results: Option<SearchResponse>,
    pub onsignal: Option<Callback<RoutingMessage>>,
    task: Option<FetchTask>,
}

#[derive(PartialEq, Clone)]
pub struct SearchProps {
    pub onsignal: Option<Callback<RoutingMessage>>,
    pub trackname: Option<String>,
}

impl Default for SearchProps {
    fn default() -> Self {
        SearchProps {
            onsignal: None,
            trackname: None,
        }
    }
}

impl Component<Registry> for SearchModel {
    type Message = SearchMessage;
    type Properties = SearchProps;

    fn create(props: Self::Properties, _context: &mut Env<Registry, Self>) -> Self {
        SearchModel {
            search_str: "".into(),
            search_results: None,
            onsignal: props.onsignal,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        match msg {
            SearchMessage::UpdateSearchString(s) => {
                self.search_str = s;
                true
            }
            SearchMessage::Search => {
                self.search_results = None;
                let callback = env.send_back(SearchMessage::ResultsReceived);
                self.task = Some(env.track.search(&self.search_str, None, callback));
                true
            }
            SearchMessage::LoadMore { term, continuation_token } => {
                let callback = env.send_back(SearchMessage::ResultsReceived);
                self.task = Some(env.track.search(&term, Some(&continuation_token), callback));
                false
            }
            SearchMessage::ResultsReceived(r) => {
                self.task = None;
                let old_results = self.search_results.take();
                let results = match old_results {
                    Some(l) => SearchResponse::combine(l, r),
                    None => r,
                };
                self.search_results = Some(results);
                true
            }
            SearchMessage::Route(message) => {
                if let Some(callback) = &self.onsignal {
                    callback.emit(message);
                }
                false
            }
            SearchMessage::UnknownKey => false,
        }
    }
}
