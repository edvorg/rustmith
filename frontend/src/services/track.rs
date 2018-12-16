use rustmith_common::track::Action;
use rustmith_common::track::Track;
use yew::prelude::Callback;

pub enum TrackLoadResult {
    Loaded(Track),
    #[allow(dead_code)]
    Error,
}

pub trait TrackService {
    fn load_track(&self, song_id: &str, callback: Callback<TrackLoadResult>);
}

pub struct StubTrackService {}

impl TrackService for StubTrackService {
    fn load_track(&self, _song_id: &str, callback: Callback<TrackLoadResult>) {
        let actions = vec![
            Action::fret_action(2000, 2200, 10, 3),
            Action::fret_action(2400, 2600, 10, 3),
            Action::fret_action(2800, 3000, 10, 3),
            Action::fret_action(3200, 4200, 10, 3),
            Action::fret_action(4400, 5400, 10, 3),
            Action::fret_action(5600, 5800, 9, 3),
            Action::fret_action(6000, 6200, 9, 3),
            Action::fret_action(6400, 6600, 10, 3),
            Action::fret_action(6800, 7000, 9, 3),
            Action::fret_action(7200, 7400, 7, 3),
            Action::fret_action(7600, 7800, 10, 4),
            Action::fret_action(8000, 8200, 1, 4),
            Action::fret_action(8400, 8600, 2, 4),
            Action::fret_action(8800, 9000, 3, 4),
            Action::fret_action(9200, 10400, 4, 4),
        ];
        callback.emit(TrackLoadResult::Loaded(Track { actions }))
    }
}

impl Default for StubTrackService {
    fn default() -> Self {
        StubTrackService {}
    }
}
