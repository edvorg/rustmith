use yew::prelude::*;
use context::Context;
use std::time::Duration;
use yew::services::Task;
use stdweb::web::{
    document,
    IParentNode,
};
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLUniformLocation,
    WebGLBuffer
};

pub enum GameMessage {
    Animate,
}

pub struct GameModel {
    onsignal: Option<Callback<Context>>,
    job: Box<Task>,
    canvas: Option<CanvasElement>,
    ctx: Option<gl>,
}

#[derive(PartialEq, Clone)]
pub struct GameProps {
    pub onsignal: Option<Callback<Context>>,
}

impl Default for GameProps {
    fn default() -> Self {
        GameProps {
            onsignal: None
        }
    }
}

impl Component<Context> for GameModel {
    type Message = GameMessage;
    type Properties = GameProps;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.log("creating game model");
        GameModel {
            onsignal: props.onsignal,
            job: GameModel::animate(context),
            canvas: None,
            ctx: None,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> bool {
        match msg {
            GameMessage::Animate => {
                self.setup_graphics(context);
                self.ctx.as_mut().map(GameModel::render);
                self.job = GameModel::animate(context);
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties, context: &mut Env<Context, Self>) -> bool {
        false
    }
}

impl Renderable<Context, GameModel> for GameModel {
    fn view(&self) -> Html<Context, GameModel> {
        html! {
          <canvas id="canvas", width=640, height=480,></canvas>
        }
    }
}

impl GameModel {
    fn animate(context: &mut Env<Context, Self>) -> Box<Task> {
        let send_back = context.send_back(|_| GameMessage::Animate);
        Box::new(context.timeout.spawn(Duration::from_millis(1000 / 60 as u64), send_back))
    }

    fn setup_graphics(&mut self, context: &mut Env<Context, Self>) {
        if self.canvas.is_none() {
            context.console.log("Setting up graphics context");
            match document().query_selector("#canvas") {
                Ok(Some(canvas)) => {
                    let canvas: CanvasElement = canvas.try_into().unwrap();
                    let ctx: gl = canvas.get_context().unwrap();
                    context.console.log("Graphics context inititalized");
                    ctx.clear_color(1.0, 0.0, 0.0, 1.0);
                    ctx.clear(gl::COLOR_BUFFER_BIT);
                    self.canvas = Some(canvas);
                    self.ctx = Some(ctx);
                    ()
                },
                _ => (),
            }
        }
    }

    fn render(ctx: &mut gl) {
        ctx.clear(gl::COLOR_BUFFER_BIT);
    }
}