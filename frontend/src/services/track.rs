use rustmith_common::track::fret_action;
use rustmith_common::track::hand_position;
use rustmith_common::track::Track;
use std::str::FromStr;
use yew::prelude::Callback;

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

pub enum TrackLoadResult {
    Loaded(Track),
    #[allow(dead_code)]
    Error,
}

pub enum TrackCreateResult {
    Created(String, Track),
    #[allow(dead_code)]
    Error,
}

pub trait TrackService {
    fn create_track(&mut self, name: &str, youtube_id: &str, content: &str, callback: Callback<TrackCreateResult>);
    fn load_track(&self, track_id: &str, callback: Callback<TrackLoadResult>);
    fn search(&self, term: &str, continuation_token: Option<&String>, callback: Callback<SearchResponse>);
}

pub struct StubTrackService {
    track: Track,
    items: Vec<SearchItem>,
}

impl TrackService for StubTrackService {
    fn create_track(&mut self, name: &str, youtube_id: &str, content: &str, callback: Callback<TrackCreateResult>) {
        let id = self.items.len().to_string();
        let result = Track::parse(content)
            .map(|t| TrackCreateResult::Created(id, t))
            .unwrap_or_else(|_| TrackCreateResult::Error);
        if let TrackCreateResult::Created(id, track) = &result {
            self.track = track.clone();
            self.items.push(SearchItem {
                name: String::from(name),
                id: id.clone(),
                url: make_youtube_url(youtube_id),
            })
        }
        callback.emit(result);
    }

    fn load_track(&self, _track_id: &str, callback: Callback<TrackLoadResult>) {
        callback.emit(TrackLoadResult::Loaded(self.track.clone()))
    }

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

impl Default for StubTrackService {
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
                url: make_youtube_url(id),
                id: internal_id.to_string(),
            })
            .collect();

        StubTrackService {
            track: StubTrackService::make_stub_track(),
            items,
        }
    }
}

impl StubTrackService {
    fn make_stub_track() -> Track {
        let actions = vec![
            fret_action(2000, 2200, 10, 3),
            fret_action(2400, 2600, 10, 3),
            fret_action(2800, 3000, 10, 3),
            fret_action(3200, 4200, 10, 3),
            fret_action(4400, 5400, 10, 3),
            fret_action(5600, 5800, 9, 3),
            fret_action(6000, 6200, 9, 3),
            fret_action(6400, 6600, 10, 3),
            fret_action(6800, 7000, 9, 3),
            fret_action(7200, 7400, 7, 3),
            fret_action(7600, 7800, 10, 4),
            fret_action(8000, 8200, 1, 4),
            fret_action(8400, 8600, 2, 4),
            fret_action(8800, 9000, 3, 4),
            fret_action(9200, 10400, 4, 1),
            fret_action(9200, 10400, 4, 2),
            fret_action(9200, 10400, 4, 3),
            fret_action(9200, 10400, 4, 4),
            fret_action(9200, 10400, 4, 5),
            fret_action(9200, 10400, 4, 6),
            fret_action(10000, 10500, 1, 5),
            fret_action(11000, 11500, 10, 6),
            fret_action(12000, 12500, 1, 5),
            fret_action(13000, 13500, 10, 6),
            fret_action(14000, 14500, 1, 5),
            fret_action(15000, 15500, 10, 6),
        ];
        let hand_positions = vec![
            hand_position(0, 7),
            hand_position(7600, 7),
            hand_position(8000, 1),
            hand_position(10000, 1),
            hand_position(11000, 10),
            hand_position(12000, 1),
            hand_position(13000, 10),
            hand_position(14000, 1),
            hand_position(15000, 10),
            hand_position(99_999_999, 1),
        ];
        Track { actions, hand_positions }
    }
}

pub fn make_youtube_url(id: &str) -> String {
    format!("https://www.youtube.com/embed/{}?autoplay=1&loop=1", id)
}
