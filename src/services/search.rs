use yew::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct SearchItem {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub enum SearchResponse {
    Result { term: String, items: Vec<SearchItem>, continuation_token: Option<String> },
    Error,
}

impl SearchResponse {
    pub fn combine(left: SearchResponse, right: SearchResponse) -> SearchResponse {
        match (left, right) {
            (SearchResponse::Error, _) => SearchResponse::Error,
            (l @ SearchResponse::Result { term: _, items: _, continuation_token: _ }, SearchResponse::Error) => l,
            (SearchResponse::Result { term: left_term @ _, items: mut left_items @ _, continuation_token: _ }, SearchResponse::Result { term: right_term @ _, items: mut right_items @ _, continuation_token: right_token @ _ }) => {
                if left_term == right_term {
                    left_items.append(right_items.as_mut());
                    SearchResponse::Result {
                        term: left_term,
                        items: left_items,
                        continuation_token: right_token
                    }
                } else {
                    SearchResponse::Result {
                        term: right_term,
                        items: right_items,
                        continuation_token: right_token
                    }
                }
            },
        }
    }
}

pub trait SearchService {
    fn search(&self, term: &str, continuation_token: Option<&String>, callback: Callback<SearchResponse>);
}

pub struct StubSearchService {
    items: Vec<SearchItem>,
}

impl StubSearchService {
    pub fn new() -> StubSearchService {
        let songs = vec!(
            "Jimi Hendrix - Foxey Lady",
            "Queen - Bohemian Rhapsody",
            "Guns N' Roses - Welcome To The Jungle",
            "Guns N' Roses - Knockin' On Heaven's Door",
            "AC/DC - Highway to Hell",
            "Michael Jackson - Beat It",
            "Daft Punk - Get Lucky",
            "Scorpions - Wind Of Change",
            "The Cranberries - Zombie",
            "The Police - Every Breath You Take",
            "Dire Straits - Sultans Of Swing",
            "Led Zeppelin - Stairway To Heaven",
            "Metallica - Nothing Else Matters",
            "Black Sabbath - Paranoid",
            "Ozzy Osbourne - Crazy Train",
            "Lynyrd Skynyrd - Sweet Home Alabama",
        );
        let items = songs.into_iter()
            .map(|s| -> String { s.into() })
            .zip(0..)
            .map(move |(a, b)| {
                SearchItem {
                    name: a.into(),
                    id: b.to_string(),
                }
            }).collect();
        StubSearchService { items }
    }
}

impl SearchService for StubSearchService {
    fn search(&self, term: &str, continuation_token: Option<&String>, callback: Callback<SearchResponse>) {
        let lowercase_term = String::from(term).to_lowercase();
        let batch_size = 3;
        let results = self.items.iter()
            .filter(|i| i.name.to_lowercase().contains(&lowercase_term));
        let skip = continuation_token.map(|t| usize::from_str(t).unwrap()).unwrap_or(0);
        let results_at_cursor: Vec<&SearchItem> = results.skip(skip).collect();
        let response = if results_at_cursor.len() > batch_size {
            SearchResponse::Result {
                term: term.into(),
                items: results_at_cursor.into_iter().map(|i| i.clone()).take(batch_size).collect(),
                continuation_token: Some((skip + batch_size).to_string()),
            }
        } else {
            SearchResponse::Result {
                term: term.into(),
                items: results_at_cursor.into_iter().map(|i| i.clone()).collect(),
                continuation_token: None,
            }
        };
        callback.emit(response);
    }
}
