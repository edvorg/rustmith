use yew::prelude::*;
use crate::registry::Registry;
use yew::services::Task;
use stdweb::web::{
    document,
    IParentNode,
};
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::IHtmlElement;
use crate::graphics::renderer;
use crate::graphics::renderer::Renderer;
use stdweb::web::window;
use stdweb::web::event::ResizeEvent;
use stdweb::web::IEventTarget;
use crate::services::audio::{
    Gain,
    Oscillator,
    Destination,
    AudioNode,
    MediaStreamSource,
};
use crate::services::ext::HiDPI;
use crate::fps::FpsStats;
use crate::fps::FpsModel;

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
    ConnectMicrophone(MediaStreamSource)
}

struct GameStats {
    notes_missed: u16,
    notes_hit: u16,
    mastery: u16,
}

pub struct GameModel {
    job: Box<Task>,
    renderer: Option<renderer::Renderer>,
    last_time: Option<f64>,
    pub on_signal: Option<Callback<RoutingMessage>>,
    pub song_id: Option<String>,
    pub song_url: Option<String>,
    stats: GameStats,
    oscillator: Oscillator,
    gain: Gain,
    destination: Destination,
    playing: bool,
    mic: Option<MediaStreamSource>,
    fps: FpsStats,
    fps_snapshot: FpsStats,
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
        let callback = env.send_back(|source| {
            return GameMessage::ConnectMicrophone(source);
        });
        env.audio.create_media_stream_source_audio(callback);
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
                mastery: 0
            },
            oscillator,
            gain,
            destination,
            playing: false,
            mic: None,
            fps: FpsStats::new(),
            fps_snapshot: FpsStats::new(),
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
            },
            GameMessage::Exit => {
                if let Some(callback) = &self.on_signal {
                    callback.emit(RoutingMessage::ExitGame);
                }
                false
            },
            GameMessage::Resize((width, height)) => {
                env.console.log(&format!("Canvas resized ({}, {})", width, height));
                if let Some(r) = &mut self.renderer {
                    r.set_viewport(width, height);
                }
                false
            },
            GameMessage::ToggleE => {
                self.playing = !self.playing;
                if self.playing {
                    self.gain.set_value(0.1);
                } else {
                    self.gain.set_value(0.0);
                }
                false
            },
            GameMessage::ConnectMicrophone(mic) => {
                env.console.log("Established mic connection");
                js! { use_stream(@{&mic.js()}); };
                self.mic = Some(mic);
                false
            },
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
          </div>
        }
    }
}

impl GameModel {
    fn animate(env: &mut Env<Registry, Self>) -> Box<Task> {
        let send_back = env.send_back(|time| GameMessage::Animate { time });
        Box::new(env.render.request_animation_frame(send_back))
    }

    fn update_canvas(canvas: &mut CanvasElement) {
        let real_to_css_pixels = CanvasElement::device_pixel_ratio();
        let display_width  = (canvas.client_width() * real_to_css_pixels).floor() as u32;
        let display_height = (canvas.client_height() * real_to_css_pixels).floor() as u32;
        if canvas.width()  != display_width || canvas.height() != display_height {
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
            },
            _ => None,
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
