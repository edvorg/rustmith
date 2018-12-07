#[derive(Serialize, Clone)]
pub struct Note {
    pub frequency: f64,
    pub name: String,
}

js_serializable!( Note );

impl Note {
    pub fn make_test_frequencies() -> Vec<Note> {
        let r: Vec<_> = (0..30).collect();
        r.into_iter().flat_map(|i| {
            let c2 = 65.41f64;
            let notes = vec!("C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B");
            let frequency = c2 * 2.0f64.powf(i as f64 / 12.0);
            let name = String::from(notes[i % 12]);
            let above_name = format!("{} (a bit sharp)", &name);
            let below_name = format!("{} (a bit flat)", &name);
            let note = Note {
                frequency,
                name,
            };
            let just_above = Note {
                frequency: frequency * 2.0f64.powf(1.0 / 48.0),
                name: above_name
            };
            let just_below = Note {
                frequency: frequency * 2.0f64.powf(-1.0 / 48.0),
                name: below_name
            };
            vec!(just_below, note, just_above)
        }).collect()
    }
}