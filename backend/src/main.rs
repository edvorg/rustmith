use rustmith_common::note::Note;

fn main() {
    let freqs = Note::make_test_frequencies();
    let s = serde_json::to_string_pretty(&freqs).unwrap();
    println!("frequences\n{}", s);
}
