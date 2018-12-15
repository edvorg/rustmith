#[macro_use]
extern crate stdweb;

use stdweb::unstable::TryInto;
use stdweb::Value;
use rustmith_common::Note;

fn compute_correlations(timeseries: Vec<f64>, sample_rate: f64) -> Vec<Vec<f64>> {
    let test_frequencies: Vec<Note> = Note::make_test_frequencies();
    // 2pi * frequency gives the appropriate period to sine.
    // timeseries index / sample_rate gives the appropriate time coordinate.
    let scale_factor = 2.0 * std::f64::consts::PI / sample_rate;
    test_frequencies
        .into_iter()
        .map(|f| {
            let frequency = f.frequency;
            // Represent a complex number as a length-2 array [ real, imaginary ].
            let mut accumulator: Vec<f64> = vec![0.0, 0.0];
            for (t, item) in timeseries.iter().enumerate() {
                accumulator[0] += item * (scale_factor * frequency * (f64::from(t as u32))).cos();
                accumulator[1] += item * (scale_factor * frequency * (f64::from(t as u32))).cos();
            }
            accumulator
        })
        .collect()
}

fn main() {
    let callback = |timeseries: Value, sample_rate: Value| {
        let timeseries: Vec<f64> = timeseries.try_into().unwrap();
        let sample_rate: f64 = sample_rate.try_into().unwrap();
        compute_correlations(timeseries, sample_rate)
    };
    js! {
      self.onmessage = function(event) {
        var callback = @{callback};
        var amplitudes = callback(event.data.timeseries, event.data.sample_rate);
        self.postMessage({ "frequency_amplitudes": amplitudes });
      };
    }
}
