use crate::registry::Registry;
use crate::services::search::SearchItem;
use crate::services::search::SearchResponse;
use crate::services::search::SearchService;
use yew::prelude::*;

pub enum RoutingMessage {
    /// switch to game screen and load song with id song_id
    StartGame { song_id: String, song_url: String },
}

#[derive(Debug)]
pub enum SearchMessage {
    UpdateSearchString(String),
    Search,
    UnknownKey,
    ResultsReceived(SearchResponse),
    LoadMore { term: String, continuation_token: String },
    PlayGame { song_id: String, song_url: String },
}

pub struct SearchModel {
    pub search_str: String,
    pub search_results: Option<SearchResponse>,
    pub onsignal: Option<Callback<RoutingMessage>>,
}

#[derive(PartialEq, Clone)]
pub struct SearchProps {
    pub onsignal: Option<Callback<RoutingMessage>>,
}

impl Default for SearchProps {
    fn default() -> Self {
        SearchProps { onsignal: None }
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
                let callback = env.send_back(|r| SearchMessage::ResultsReceived(r));
                env.search.search(&self.search_str, None, callback);
                true
            }
            SearchMessage::LoadMore { term, continuation_token } => {
                let callback = env.send_back(|r| SearchMessage::ResultsReceived(r));
                env.search.search(&term, Some(&continuation_token), callback);
                false
            }
            SearchMessage::ResultsReceived(r) => {
                let old_results = self.search_results.take();
                let results = match old_results {
                    Some(l) => SearchResponse::combine(l, r),
                    None => r,
                };
                self.search_results = Some(results);
                true
            }
            SearchMessage::PlayGame { song_id, song_url } => {
                if let Some(callback) = &self.onsignal {
                    callback.emit(RoutingMessage::StartGame { song_id, song_url });
                }
                false
            }
            SearchMessage::UnknownKey => false,
        }
    }
}

impl Renderable<Registry, SearchModel> for SearchModel {
    fn view(&self) -> Html<Registry, SearchModel> {
        html! {
        <div>
        <div id="logo-block",>
          <div id="logo",> { "rustmith" } </div>
          <input id="search",
                 type="text",
                 placeholder="Search for a song. E.g. 'Queen - Bohemian Rhapsody'",
                 oninput=|e| SearchMessage::UpdateSearchString(e.value),
                 onkeypress=|e| {
                       if e.key() == "Enter" {
                         SearchMessage::Search
                       } else {
                         SearchMessage::UnknownKey
                       }
                 },
                 autofocus=true,>
            { "search" }
          </input>
        </div>
        <div id="search-results-block",>
        { self.results_view() }
        </div>
        </div>
        }
    }
}

impl SearchModel {
    fn item_view(&self, item: &SearchItem) -> Html<Registry, SearchModel> {
        let id = item.id.clone();
        let url = item.url.clone();
        let name = item.name.clone();
        html! {
          <div>
            { name }
            <button onclick=|_| SearchMessage::PlayGame { song_id: id.clone(), song_url: url.clone() },> { "play" } </button>
          </div>
        }
    }

    fn results_view(&self) -> Html<Registry, SearchModel> {
        match &self.search_results {
            Some(SearchResponse::Result {
                term: _,
                items,
                continuation_token: _,
            }) if items.is_empty() => {
                html! {
                    <div> { "Song not found" } </div>
                }
            }
            Some(SearchResponse::Result {
                term,
                items,
                continuation_token: Some(continuation_token),
            }) => {
                let term = term.clone();
                let continuation_token = continuation_token.clone();
                html! {
                  <div>
                    <div> { for items.iter().map(|i| self.item_view(i)) } </div>
                    <button onclick=|_| SearchMessage::LoadMore {
                              term: term.clone(),
                              continuation_token: continuation_token.clone(),
                            },>
                      { "Load more" }
                    </button>
                  </div>
                }
            }
            Some(SearchResponse::Result {
                term: _,
                items,
                continuation_token: _,
            }) => {
                html! {
                    <div> { for items.iter().map(|i| self.item_view(i)) } </div>
                }
            }
            Some(SearchResponse::Error) => {
                html! {
                    <div> { "Search failed" } </div>
                }
            }
            None => {
                html! {
                    <div> { "Let's rock!" } </div>
                }
            }
        }
    }
}
