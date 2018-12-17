use rustmith_common::track::fret_action;
use rustmith_common::track::hand_position;
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
        callback.emit(TrackLoadResult::Loaded(Track { actions, hand_positions }))
    }
}

impl Default for StubTrackService {
    fn default() -> Self {
        StubTrackService {}
    }
}
