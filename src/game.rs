use yew::prelude::*;
use registry::Registry;
use yew::services::Task;
use stdweb::web::{
    document,
    IParentNode,
};
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;

use graphics::renderer;
use graphics::renderer::Renderer;

/// this type of message is used for inter-component communication
pub enum RoutingMessage {
    /// switch to search screen
    ExitGame,
}

pub enum GameMessage {
    Animate { time: f64 },
    Exit,
}

pub struct GameModel {
    job: Box<Task>,
    renderer: Option<renderer::Renderer>,
    last_time: Option<f64>,
    pub on_signal: Option<Callback<RoutingMessage>>,
    pub song_id: Option<String>,
    pub song_url: Option<String>,
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
        GameModel {
            job: GameModel::animate(env),
            renderer: None,
            last_time: None,
            on_signal: props.onsignal,
            song_id: props.songid,
            song_url: props.songurl,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        match msg {
            GameMessage::Animate { time } => {
                if self.renderer.is_none() {
                    self.renderer = self.setup_graphics(env);
                }
                if let Some(r) = &mut self.renderer {
                    r.render((self.last_time.unwrap_or(time) - time) / 1000.0);
                }
                self.job = GameModel::animate(env);
                self.last_time = Some(time);
                false
            },
            GameMessage::Exit => {
                if let Some(callback) = &self.on_signal {
                    callback.emit(RoutingMessage::ExitGame);
                }
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
        let url = self.song_url.clone().unwrap();
        html! {
          <div>
            <canvas id="canvas", width=640, height=480,></canvas>
            <iframe width="640",
                    height="480",
                    src=url.clone(),
                    frameborder="0",
                    allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture",>
            </iframe>
            <button onclick = |_| GameMessage::Exit ,> { "exit" } </button>
          </div>
        }
    }
}

impl GameModel {
    fn animate(env: &mut Env<Registry, Self>) -> Box<Task> {
        let send_back = env.send_back(|time| GameMessage::Animate { time });
        Box::new(env.render.request_animation_frame(send_back))
    }

    fn setup_graphics(&self, env: &mut Env<Registry, Self>) -> Option<Renderer> {
        env.console.log("Setting up graphics context");
        match document().query_selector("#canvas") {
            Ok(Some(canvas)) => {
                let canvas: CanvasElement = canvas.try_into().unwrap();
                let renderer = Renderer::new(&canvas, canvas.width(), canvas.height());
                env.console.log("Graphics context inititalized");
                Some(renderer)
            },
            _ => None,
        }
    }
}