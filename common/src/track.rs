use std::num::ParseIntError;
use std::time::Duration;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Fret {
    pub fret: u8,
    pub string: u8,
    pub ends_at: Duration,
    pub starts_at: Duration,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Interval {
    HalfStep,
    Step,
    DoubleStep,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Action {
    Fret(Fret),
    Slide(Fret, Fret),
    Bend(Fret, Interval),
}

impl Action {
    pub fn starts_at(&self) -> &Duration {
        match self {
            Action::Fret(f) => &f.starts_at,
            Action::Slide(f1, f2) => ::std::cmp::min(&f1.starts_at, &f2.starts_at),
            Action::Bend(f, _) => &f.starts_at,
        }
    }

    pub fn ends_at(&self) -> &Duration {
        match self {
            Action::Fret(f) => &f.ends_at,
            Action::Slide(f1, f2) => ::std::cmp::max(&f1.ends_at, &f2.ends_at),
            Action::Bend(f, _) => &f.ends_at,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HandPosition {
    pub fret: u8,
    pub at: Duration,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TrackData {
    pub actions: Vec<Action>,
    pub hand_positions: Vec<HandPosition>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub youtube_id: String,
    pub data: TrackData,
}

pub struct TrackView<'a> {
    pub actions: Vec<&'a Action>,
    pub hand_positions: Vec<&'a HandPosition>,
}

impl TrackData {
    pub fn parse(content: &str) -> Result<TrackData, ParseIntError> {
        let mut actions: Vec<Action> = vec![];
        let mut hand_positions: Vec<HandPosition> = vec![];
        let lines: Vec<&str> = content.split('\n').collect();
        for line in lines {
            let segments: Vec<&str> = line.split(':').collect();
            match segments.first() {
                Some(&"fret") => actions.push(fret_action(
                    segments[1].parse::<u64>()?,
                    segments[2].parse::<u64>()?,
                    segments[3].parse::<u8>()?,
                    segments[4].parse::<u8>()?,
                )),
                Some(&"hand") => hand_positions.push(hand_position(segments[1].parse::<u64>()?, segments[2].parse::<u8>()?)),
                _ => (),
            }
        }
        Result::Ok(TrackData { actions, hand_positions })
    }

    pub fn view(&self, from: Duration) -> TrackView {
        let until = from + Duration::from_secs(60);
        let actions = self.actions.iter().filter(|a| from <= *a.starts_at() && *a.ends_at() <= until).collect();
        let hand_positions = self.hand_positions.iter().filter(|p| from <= p.at).collect();
        TrackView { actions, hand_positions }
    }
}

pub fn fret_action(starts_at: u64, ends_at: u64, fret: u8, string: u8) -> Action {
    Action::Fret(Fret {
        fret,
        string,
        starts_at: Duration::from_millis(starts_at),
        ends_at: Duration::from_millis(ends_at),
    })
}

pub fn hand_position(at: u64, fret: u8) -> HandPosition {
    HandPosition {
        fret,
        at: Duration::from_millis(at),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchItem {
    pub name: String,
    pub id: String,
    pub youtube_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchResponse {
    Result {
        term: String,
        items: Vec<SearchItem>,
        continuation_token: Option<String>,
    },
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

#[derive(Serialize, Deserialize)]
pub enum TrackLoadResult {
    Loaded(TrackData),
    Error,
}

#[derive(Serialize, Deserialize)]
pub enum TrackCreateResult {
    Created(String, Track),
    Error,
}

#[derive(Debug)]
pub enum ApiError {
    DatabaseError,
    InvalidFormatError,
}
