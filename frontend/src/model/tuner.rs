use crate::registry::Registry;
use crate::services::worker::Worker;
use rustmith_common::note::Note;
use std::time::Duration;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::IElement;
use stdweb::Value;
use yew::prelude::Component;
use yew::prelude::Env;
use yew::services::Task;
use yew_audio::AudioNode;
use yew_audio::AudioProcessingEvent;
use yew_audio::Destination;
use yew_audio::Gain;
use yew_audio::MediaStreamSource;
use yew_audio::Oscillator;
use yew_audio::ScriptProcessor;

static SAMPLE_LENGTH_MILLIS: u32 = 100;

pub struct TunerModel {
    mic: Option<MediaStreamSource>,
    destination: Destination,
    #[allow(dead_code)]
    oscillator: Oscillator,
    gain: Gain,
    script_processor: Option<ScriptProcessor>,
    correlation_worker: Option<Worker>,
    buffer: Vec<f64>,
    recording: bool,
    test_frequencies: Vec<Note>,
    pub note: Option<Note>,
    recording_job: Option<Box<dyn Task>>,
    playing: bool,
}

pub enum TunerMessage {
    InterpretCorrelation(Value),
    AudioProcess(AudioProcessingEvent),
    ContinueAudioProcess,
    ToggleE,
}

#[derive(PartialEq, Clone)]
pub struct TunerProps {
    pub mic: Option<MediaStreamSource>,
}

impl Default for TunerProps {
    fn default() -> Self {
        TunerProps { mic: None }
    }
}

impl Component<Registry> for TunerModel {
    type Message = TunerMessage;
    type Properties = TunerProps;

    fn create(props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        let oscillator = env.audio.create_oscillator();
        oscillator.set_frequency(440.0);

        let gain = env.audio.create_gain();
        gain.set_value(0.0);

        let destination = env.audio.destination();

        oscillator.connect(&gain);
        gain.connect(&destination);
        oscillator.start();

        TunerModel {
            mic: props.mic,
            destination,
            oscillator,
            gain,
            script_processor: None,
            correlation_worker: None,
            buffer: vec![],
            recording: false,
            test_frequencies: Note::make_test_frequencies(),
            note: None,
            recording_job: None,
            playing: false,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        match msg {
            TunerMessage::InterpretCorrelation(e) => {
                let frequency_amplitudes: Vec<Vec<f64>> = js! (
                    return @{&e}.data.frequency_amplitudes;
                )
                .try_into()
                .unwrap();
                // Compute the (squared) magnitudes of the complex amplitudes for each
                // test frequency.
                let magnitudes: Vec<f64> = frequency_amplitudes.into_iter().map(|v| v[0] * v[0] + v[1] * v[1]).collect();
                // Find the maximum in the list of magnitudes.
                let mut maximum_index = -1i32;
                let mut maximum_magnitude = 0.0f64;
                for i in 0..(magnitudes.len() as i32) {
                    if magnitudes[i as usize] <= maximum_magnitude {
                        continue;
                    }
                    maximum_index = i;
                    maximum_magnitude = magnitudes[i as usize];
                }
                // Compute the average magnitude. We'll only pay attention to frequencies
                // with magnitudes significantly above average.
                let average: f64 = magnitudes.iter().sum();
                let average = average / (f64::from(magnitudes.len() as u32));
                let confidence = maximum_magnitude / average;
                let confidence_threshold = 10.0; // empirical, arbitrary.
                if confidence > confidence_threshold {
                    let dominant_frequency = &self.test_frequencies[maximum_index as usize];
                    self.note = Some(dominant_frequency.clone());
                    true
                } else {
                    false
                }
            }
            TunerMessage::AudioProcess(v) => {
                if !self.recording {
                    return false;
                }
                self.buffer.append(&mut v.input_buffer().get_channel_data_buffer(0));
                let sample_rate = env.audio.sample_rate();
                if self.buffer.len() <= (f64::from(SAMPLE_LENGTH_MILLIS) * sample_rate / 1000.0) as usize {
                    return false;
                }
                self.recording = false;
                if let Some(w) = &self.correlation_worker {
                    w.post_message(js! {
                        return {
                            "timeseries": @{&self.buffer},
                            "test_frequencies": window.test_frequencies,
                            "sample_rate": @{sample_rate},
                        };
                    });
                    self.buffer.clear();
                    let delay = env.send_back(|_| TunerMessage::ContinueAudioProcess);
                    self.recording_job = Some(Box::new(env.timeout.spawn(Duration::from_millis(250), delay)));
                } else {
                    env.console.warn("Something is wrong, correlation worker not found");
                }
                false
            }
            TunerMessage::ContinueAudioProcess => {
                self.recording = true;
                false
            }
            TunerMessage::ToggleE => {
                self.playing = !self.playing;
                if self.playing {
                    self.gain.set_value(0.1);
                } else {
                    self.gain.set_value(0.0);
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties, env: &mut Env<Registry, Self>) -> bool {
        match props.mic {
            Some(mic) => {
                let worker_path = document().body().unwrap().get_attribute("data-correlation-worker").unwrap();

                let correlation_worker = Worker::new(&worker_path);
                let on_event = env.send_back(TunerMessage::InterpretCorrelation);
                correlation_worker.add_event_listener("message", on_event);

                let script_processor = env.audio.create_script_processor(1024, 1, 1);
                script_processor.connect(&self.destination);
                script_processor.set_onaudioprocess(env.send_back(TunerMessage::AudioProcess));

                mic.connect(&script_processor);

                self.script_processor = Some(script_processor);
                self.correlation_worker = Some(correlation_worker);
                self.recording = true;
                self.buffer.clear();
                self.mic = Some(mic);

                false
            }
            None => false,
        }
    }
}

impl Drop for TunerModel {
    fn drop(&mut self) {
        self.gain.set_value(0.0);
    }
}
