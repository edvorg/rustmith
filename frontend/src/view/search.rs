use crate::model::search::*;
use yew::prelude::*;
use crate::registry::Registry;
use rustmith_common::track::SearchItem;
use rustmith_common::track::SearchResponse;
use crate::services::track::make_youtube_url;

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
        <button onclick=|_| SearchMessage::Route(RoutingMessage::NewSong),> { "Create song" } </button>
        </div>
        </div>
        }
    }
}

impl SearchModel {
    fn item_view(&self, item: &SearchItem) -> Html<Registry, SearchModel> {
        let id = item.id.clone();
        let url = make_youtube_url(&item.youtube_id);
        let name = item.name.clone();
        html! {
          <div>
            { name }
            <button onclick=|_| SearchMessage::Route(RoutingMessage::StartGame { song_id: id.clone(), song_url: url.clone() }),> { "play" } </button>
          </div>
        }
    }

    fn results_view(&self) -> Html<Registry, SearchModel> {
        match &self.search_results {
            Some(SearchResponse::Result { items, .. }) if items.is_empty() => {
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
            Some(SearchResponse::Result { items, .. }) => {
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
