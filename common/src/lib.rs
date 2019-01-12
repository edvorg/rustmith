#[macro_use]
extern crate serde_derive;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

pub mod ext;
pub mod note;
pub mod track;

#[cfg(test)]
mod tests {
    use crate::track::fret_action;
    use crate::track::TrackData;
    use std::time::Duration;

    #[test]
    fn test_view_1() {
        let actions = vec![
            fret_action(2000, 2200, 10, 3),
            fret_action(2400, 2600, 10, 3),
            fret_action(2800, 3000, 10, 3),
            fret_action(3200, 4200, 10, 3),
            fret_action(4400, 5400, 10, 3),
        ];
        let hand_positions = vec![];
        let track = TrackData { actions, hand_positions };
        assert_eq!(0, track.view(Duration::from_millis(4500)).actions.len());
        assert_eq!(1, track.view(Duration::from_millis(3300)).actions.len());
        assert_eq!(2, track.view(Duration::from_millis(2900)).actions.len());
        assert_eq!(3, track.view(Duration::from_millis(2500)).actions.len());
        assert_eq!(4, track.view(Duration::from_millis(2100)).actions.len());
        assert_eq!(5, track.view(Duration::from_millis(0)).actions.len());
        for action in &track.actions {
            assert_eq!(true, *action.starts_at() < *action.ends_at())
        }
    }
}
