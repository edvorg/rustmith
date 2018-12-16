use std::time::Duration;

#[derive(PartialEq, Clone, Debug)]
pub struct Fret {
    pub fret: u8,
    pub string: u8,
    pub ends_at: Duration,
    pub starts_at: Duration,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Interval {
    HalfStep,
    Step,
    DoubleStep,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Action {
    Fret(Fret),
    Slide(Fret, Fret),
    Bend(Fret, Interval),
}

impl Action {
    pub fn fret_action(starts_at: u64, ends_at: u64, fret: u8, string: u8) -> Action {
        Action::Fret(Fret {
            fret,
            string,
            starts_at: Duration::from_millis(starts_at),
            ends_at: Duration::from_millis(ends_at),
        })
    }

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

#[derive(PartialEq, Clone)]
pub struct Track {
    pub actions: Vec<Action>,
}

impl Track {
    pub fn view(&self, from: Duration) -> Vec<&Action> {
        let until = from + Duration::from_secs(60);
        self.actions.iter().filter(|a| from <= *a.starts_at() && *a.ends_at() <= until).collect()
    }
}
