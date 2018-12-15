use crate::fps::FpsModel;
use crate::fps::FpsStats;
use crate::graphics::renderer;
use crate::graphics::renderer::Renderer;
use crate::registry::Registry;
use crate::services::ext::CanvasElementExt;
use crate::services::ext::WindowExt;
use crate::services::worker::Worker;
use rustmith_common::Note;
use std::time::Duration;
use stdweb::unstable::TryInto;
use stdweb::web::event::ResizeEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::window;
use stdweb::web::IEventTarget;
use stdweb::web::RequestAnimationFrameHandle;
use stdweb::web::{document, IParentNode};
use stdweb::Value;
use yew::prelude::*;
use yew::services::Task;
use yew_audio::AudioProcessingEvent;
use yew_audio::MediaStream;
use yew_audio::ScriptProcessor;
use yew_audio::{AudioNode, Destination, Gain, MediaStreamSource, Oscillator};

static SAMPLE_LENGTH_MILLIS: u32 = 100;

/// this type of message is used for inter-component communication
pub enum RoutingMessage {
    /// switch to search screen
    ExitGame,
}

pub enum GameMessage {
    Animate { time: f64 },
    Resize((f32, f32)),
    Exit,
    ToggleE,
    ConnectMicrophone(MediaStream),
    AudioProcess(AudioProcessingEvent),
    ContinueAudioProcess,
    InterpretCorrelation(Value),
}

struct GameStats {
    notes_missed: u16,
    notes_hit: u16,
    mastery: u16,
}

pub struct GameModel {
    job: Box<RequestAnimationFrameHandle>,
    renderer: Option<renderer::Renderer>,
    last_time: Option<f64>,
    on_signal: Option<Callback<RoutingMessage>>,
    #[allow(dead_code)]
    song_id: Option<String>,
    song_url: Option<String>,
    stats: GameStats,
    #[allow(dead_code)]
    oscillator: Oscillator,
    gain: Gain,
    destination: Destination,
    playing: bool,
    mic: Option<MediaStreamSource>,
    script_processor: Option<ScriptProcessor>,
    correlation_worker: Option<Worker>,
    buffer: Vec<f64>,
    recording: bool,
    recording_job: Option<Box<Task>>,
    fps: FpsStats,
    fps_snapshot: FpsStats,
    note: Option<Note>,
}

#[derive(PartialEq, Clone)]
pub struct GameProps {
    pub onsignal: Option<Callback<RoutingMessage>>,
    pub songid: Option<String>,
    pub songurl: Option<String>,
}

impl Default for GameProps {
    fn default() -> Self {
        GameProps {
            onsignal: None,
            songid: None,
            songurl: None,
        }
    }
}

impl Component<Registry> for GameModel {
    type Message = GameMessage;
    type Properties = GameProps;

    fn create(props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        env.console.log("creating game model");
        let oscillator = env.audio.create_oscillator();
        let gain = env.audio.create_gain();
        let destination = env.audio.destination();
        oscillator.set_frequency(440.0);
        gain.set_value(0.0);
        oscillator.connect(&gain);
        gain.connect(&destination);
        oscillator.start();
        let on_mic = env.send_back(GameMessage::ConnectMicrophone);
        env.audio.get_user_media().call_audio(on_mic);
        GameModel {
            job: GameModel::animate(env),
            renderer: None,
            last_time: None,
            on_signal: props.onsignal,
            song_id: props.songid,
            song_url: props.songurl,
            stats: GameStats {
                notes_missed: 0,
                notes_hit: 0,
                mastery: 0,
            },
            oscillator,
            gain,
            destination,
            playing: false,
            mic: None,
            script_processor: None,
            correlation_worker: None,
            buffer: vec![],
            recording: false,
            recording_job: None,
            fps: FpsStats::new(),
            fps_snapshot: FpsStats::new(),
            note: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        match msg {
            GameMessage::Animate { time } => {
                if self.renderer.is_none() {
                    self.renderer = self.setup_graphics(env);
                }
                let delta_millis = time - self.last_time.unwrap_or(time);
                if let Some(r) = &mut self.renderer {
                    r.render(delta_millis / 1000.0);
                } else {
                    env.console.warn("Something is wrong, renderer not found");
                }
                self.job = GameModel::animate(env);
                self.last_time = Some(time);

                self.fps.log_frame(delta_millis);
                if self.fps.time > 2000.0 {
                    self.fps.drain(&mut self.fps_snapshot);
                    true
                } else {
                    false
                }
            }
            GameMessage::Exit => {
                if let Some(callback) = &self.on_signal {
                    callback.emit(RoutingMessage::ExitGame);
                } else {
                    env.console.warn("Something is wrong, router not found");
                }
                false
            }
            GameMessage::Resize((width, height)) => {
                env.console.log(&format!("Canvas resized ({}, {})", width, height));
                if let Some(r) = &mut self.renderer {
                    r.set_viewport(width, height);
                } else {
                    env.console.warn("Something is wrong, renderer not found");
                }
                false
            }
            GameMessage::ToggleE => {
                self.playing = !self.playing;
                if self.playing {
                    self.gain.set_value(0.1);
                } else {
                    self.gain.set_value(0.0);
                }
                false
            }
            GameMessage::ConnectMicrophone(mic) => {
                env.console.log("Established mic connection");
                let correlation_worker = Worker::new("rustmith_correlation_worker.js");
                let on_event = env.send_back(GameMessage::InterpretCorrelation);
                correlation_worker.add_event_listener("message", on_event);
                let mic = env.audio.create_media_stream_source(mic);
                window().set_source(&mic);
                let script_processor = env.audio.create_script_processor(1024, 1, 1);
                script_processor.connect(&self.destination);
                mic.connect(&script_processor);
                mic.connect(&self.destination);
                script_processor.set_onaudioprocess(env.send_back(GameMessage::AudioProcess));
                self.mic = Some(mic);
                self.script_processor = Some(script_processor);
                self.correlation_worker = Some(correlation_worker);
                self.recording = true;
                self.buffer.clear();
                false
            }
            GameMessage::AudioProcess(v) => {
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
                    let delay = env.send_back(|_| GameMessage::ContinueAudioProcess);
                    self.recording_job = Some(Box::new(env.timeout.spawn(Duration::from_millis(250), delay)));
                } else {
                    env.console.warn("Something is wrong, correlation worker not found");
                }
                false
            }
            GameMessage::InterpretCorrelation(e) => {
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
                    let test_frequencies: Vec<Note> = Note::make_test_frequencies();
                    let dominant_frequency = &test_frequencies[maximum_index as usize];
                    self.note = Some(dominant_frequency.clone());
                    true
                } else {
                    false
                }
            }
            GameMessage::ContinueAudioProcess => {
                self.recording = true;
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties, _env: &mut Env<Registry, Self>) -> bool {
        false
    }
}

impl Renderable<Registry, GameModel> for GameModel {
    fn view(&self) -> Html<Registry, GameModel> {
        html! {
          <div class="game",>
            <div class="game-view",>
              <button id="exit-button", onclick = |_| GameMessage::Exit ,> { "exit" } </button>
              <FpsModel: fps=&self.fps_snapshot, />
              <canvas id="canvas",></canvas>
            </div>
            <div class="game-video",>
              <iframe id="video-clip",
                      src=&self.song_url.clone().unwrap(),
                      frameborder="0",
                      allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture",>
              </iframe>
            </div>
            <div class="game-stats",>
              <div>
                { format!("Notes missed {}", &self.stats.notes_missed) }
              </div>
              <div>
                { format!("Notes hit {}", &self.stats.notes_hit) }
              </div>
              <div>
                { format!("Mastery {}%", &self.stats.mastery) }
              </div>
            </div>
            { self.effects_view() }
            { self.tuner_view() }
          </div>
        }
    }
}

impl GameModel {
    fn animate(env: &mut Env<Registry, Self>) -> Box<RequestAnimationFrameHandle> {
        let send_back = env.send_back(|time| GameMessage::Animate { time });
        let f = move |d| {
            send_back.emit(d);
        };
        Box::new(window().request_animation_frame(f))
    }

    fn update_canvas(canvas: &mut CanvasElement) {
        let real_to_css_pixels = window().device_pixel_ratio();
        let display_width = (canvas.client_width() * real_to_css_pixels).floor() as u32;
        let display_height = (canvas.client_height() * real_to_css_pixels).floor() as u32;
        if canvas.width() != display_width || canvas.height() != display_height {
            canvas.set_width(display_width);
            canvas.set_height(display_height);
        }
    }

    fn get_canvas_size(canvas: &CanvasElement) -> (f32, f32) {
        (canvas.width() as f32, canvas.height() as f32)
    }

    fn setup_graphics(&self, env: &mut Env<Registry, Self>) -> Option<Renderer> {
        env.console.log("Setting up graphics context");
        match document().query_selector("#canvas") {
            Ok(Some(canvas)) => {
                let mut canvas: CanvasElement = canvas.try_into().unwrap();
                GameModel::update_canvas(&mut canvas);
                let context = Renderer::make_cotext(&canvas);
                let size = GameModel::get_canvas_size(&canvas);
                let renderer = Renderer::new(context, size);
                let callback = env.send_back(|m| m);
                window().add_event_listener(move |_: ResizeEvent| {
                    GameModel::update_canvas(&mut canvas);
                    let size = GameModel::get_canvas_size(&canvas);
                    callback.emit(GameMessage::Resize(size));
                });
                env.console.log("Graphics context inititalized");
                Some(renderer)
            }
            _ => None,
        }
    }

    fn tuner_view(&self) -> Html<Registry, GameModel> {
        match &self.note {
            Some(n) => {
                let note_message = format!("Note: {}", n.name);
                let note_frequency = format!("Frequency: {}hz", n.frequency);
                html! {
                    <div id="game-tuner",>
                      <div>
                        { "Tuner:" }
                      </div>
                      <div id="note-name",>
                        { note_message }
                      </div>
                      <div id="frequency",>
                        { note_frequency }
                      </div>
                    </div>
                }
            }
            None => html! {
                <div id="game-tuner",>
                  <div>
                    { "Tuner:" }
                  </div>
                  <div>
                    { "Play a note" }
                  </div>
                </div>
            },
        }
    }

    fn effects_view(&self) -> Html<Registry, GameModel> {
        html! {
            <div class="game-effects",>
              <button id="note-button", onclick = |_| GameMessage::ToggleE ,> { "Play E" } </button>
              <div>
                { "Overdrive" }
              </div>
              <div>
                { "Distoration" }
              </div>
              <div>
                { "Compressor" }
              </div>
            </div>
        }
    }
}

impl Drop for GameModel {
    fn drop(&mut self) {
        self.gain.set_value(0.0);
    }
}
