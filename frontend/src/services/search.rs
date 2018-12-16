use std::str::FromStr;
use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct SearchItem {
    pub name: String,
    pub id: String,
    pub url: String,
}

#[derive(Debug)]
pub enum SearchResponse {
    Result {
        term: String,
        items: Vec<SearchItem>,
        continuation_token: Option<String>,
    },
    #[allow(dead_code)]
    Error,
}

impl SearchResponse {
    pub fn combine(left: SearchResponse, right: SearchResponse) -> SearchResponse {
        match (left, right) {
            (SearchResponse::Error, _) => SearchResponse::Error,
            (l @ SearchResponse::Result { .. }, SearchResponse::Error) => l,
            (
                SearchResponse::Result {
                    term: left_term,
                    items: mut left_items,
                    ..
                },
                SearchResponse::Result {
                    term: right_term,
                    items: mut right_items,
                    continuation_token: right_token,
                },
            ) => {
                if left_term == right_term {
                    left_items.append(right_items.as_mut());
                    SearchResponse::Result {
                        term: left_term,
                        items: left_items,
                        continuation_token: right_token,
                    }
                } else {
                    SearchResponse::Result {
                        term: right_term,
                        items: right_items,
                        continuation_token: right_token,
                    }
                }
            }
        }
    }
}

pub trait SearchService {
    fn search(&self, term: &str, continuation_token: Option<&String>, callback: Callback<SearchResponse>);
}

pub struct StubSearchService {
    items: Vec<SearchItem>,
}

impl SearchService for StubSearchService {
    fn search(&self, term: &str, continuation_token: Option<&String>, callback: Callback<SearchResponse>) {
        let lowercase_term = String::from(term).to_lowercase();
        let batch_size = 3;
        let results = self.items.iter().filter(|i| i.name.to_lowercase().contains(&lowercase_term));
        let skip = continuation_token.map(|t| usize::from_str(t).unwrap()).unwrap_or(0);
        let results_at_cursor: Vec<&SearchItem> = results.skip(skip).collect();
        let response = if results_at_cursor.len() > batch_size {
            SearchResponse::Result {
                term: term.into(),
                items: results_at_cursor.into_iter().cloned().take(batch_size).collect(),
                continuation_token: Some((skip + batch_size).to_string()),
            }
        } else {
            SearchResponse::Result {
                term: term.into(),
                items: results_at_cursor.into_iter().cloned().collect(),
                continuation_token: None,
            }
        };
        callback.emit(response);
    }
}

impl Default for StubSearchService {
    fn default() -> Self {
        let songs = vec![
            ("Jimi Hendrix - Foxey Lady", "vsI15ei76bg"),
            ("Queen - Bohemian Rhapsody", "fJ9rUzIMcZQ"),
            ("Guns N' Roses - Welcome To The Jungle", "o1tj2zJ2Wvg"),
            ("Guns N' Roses - Knockin' On Heaven's Door", "0BRUDmTxdmA"),
            ("AC/DC - Highway to Hell", "l482T0yNkeo"),
            ("Michael Jackson - Beat It", "oRdxUFDoQe0"),
            ("Daft Punk - Get Lucky", "5NV6Rdv1a3I"),
            ("Scorpions - Wind Of Change", "n4RjJKxsamQ"),
            ("The Cranberries - Zombie", "6Ejga4kJUts"),
            ("The Police - Every Breath You Take", "OMOGaugKpzs"),
            ("Dire Straits - Sultans Of Swing", "0fAQhSRLQnM"),
            ("Led Zeppelin - Stairway To Heaven", "D9ioyEvdggk"),
            ("Metallica - Nothing Else Matters", "tAGnKpE4NCI"),
            ("Black Sabbath - Paranoid", "uk_wUT1CvWM"),
            ("Ozzy Osbourne - Crazy Train", "vy1V5LHXWbg"),
            ("Lynyrd Skynyrd - Sweet Home Alabama", "ye5BuYf8q4o"),
        ];
        let items = songs
            .into_iter()
            .zip(0..)
            .map(move |((name, id), internal_id)| SearchItem {
                name: String::from(name),
                url: format!("https://www.youtube.com/embed/{}?autoplay=1&loop=1", id),
                id: internal_id.to_string(),
            })
            .collect();
        StubSearchService { items }
    }
}
