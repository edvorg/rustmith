use registry::Registry;
use yew::prelude::*;

#[derive(Debug)]
pub enum SearchMessage {
    UpdateSearch(String),
    Search,
    Blur,
    UnknownKey,
}

pub struct SearchModel {
    pub search_str: String,
}

#[derive(PartialEq, Clone)]
pub struct SearchProps {
    pub onsignal: Option<Callback<Registry>>,
}

impl Default for SearchProps {
    fn default() -> Self {
        SearchProps {
            onsignal: None,
        }
    }
}

impl Component<Registry> for SearchModel {
    type Message = SearchMessage;
    type Properties = SearchProps;

    fn create(_props: Self::Properties, _context: &mut Env<Registry, Self>) -> Self {
        SearchModel {
            search_str: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<Registry, Self>) -> bool {
        _env.console.log(&format!("updating {:?}", &msg));
        match msg {
            SearchMessage::UpdateSearch(s) => {
                self.search_str = s;
                true
            },
            SearchMessage::Blur => {
                false
            },
            SearchMessage::Search => {
                true
            },
            SearchMessage::UnknownKey => {
                false
            },
        }
    }
}

impl Renderable<Registry, SearchModel> for SearchModel {
    fn view(&self) -> Html<Registry, SearchModel> {
        html! {
        <div id="logo-block",>
          <div id="logo",> { "rustmith" } </div>
          <input id="search",
                 type="text",
                 placeholder="Search for a song. E.g. 'Queen - Bohemian Rhapsody'",
                 oninput=|e| SearchMessage::UpdateSearch(e.value),
                 onblur=|_| SearchMessage::Blur,
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
        }
    }
}
