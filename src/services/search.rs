use yew::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct SearchItem {
    pub name: String,
    pub id: String,
    pub url: String,
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
            ("Jimi Hendrix - Foxey Lady", "https://www.youtube.com/embed/vsI15ei76bg"),
            ("Queen - Bohemian Rhapsody", "https://www.youtube.com/embed/fJ9rUzIMcZQ"),
            ("Guns N' Roses - Welcome To The Jungle", "https://www.youtube.com/embed/o1tj2zJ2Wvg"),
            ("Guns N' Roses - Knockin' On Heaven's Door", "https://www.youtube.com/embed/0BRUDmTxdmA"),
            ("AC/DC - Highway to Hell", "https://www.youtube.com/embed/l482T0yNkeo"),
            ("Michael Jackson - Beat It", "https://www.youtube.com/embed/oRdxUFDoQe0"),
            ("Daft Punk - Get Lucky", "https://www.youtube.com/embed/5NV6Rdv1a3I"),
            ("Scorpions - Wind Of Change", "https://www.youtube.com/embed/n4RjJKxsamQ"),
            ("The Cranberries - Zombie", "https://www.youtube.com/embed/6Ejga4kJUts"),
            ("The Police - Every Breath You Take", "https://www.youtube.com/embed/OMOGaugKpzs"),
            ("Dire Straits - Sultans Of Swing", "https://www.youtube.com/embed/0fAQhSRLQnM"),
            ("Led Zeppelin - Stairway To Heaven", "https://www.youtube.com/embed/D9ioyEvdggk"),
            ("Metallica - Nothing Else Matters", "https://www.youtube.com/embed/tAGnKpE4NCI"),
            ("Black Sabbath - Paranoid", "https://www.youtube.com/embed/uk_wUT1CvWM"),
            ("Ozzy Osbourne - Crazy Train", "https://www.youtube.com/embed/vy1V5LHXWbg"),
            ("Lynyrd Skynyrd - Sweet Home Alabama", "https://www.youtube.com/embed/ye5BuYf8q4o"),
        );
        let items = songs.into_iter()
            .map(|(name, url)| { (String::from(name), String::from(url)) })
            .zip(0..)
            .map(move |((name, url), b)| {
                SearchItem {
                    name: name,
                    url: url + "?autoplay=1",
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
