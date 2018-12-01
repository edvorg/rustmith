use yew::prelude::*;
use context::Context;
use std::time::Duration;
use yew::services::Task;
use stdweb::web::{
    document,
    IParentNode,
    CanvasRenderingContext2d,
};
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;

pub enum GameMessage {
    Animate,
}

pub struct GameModel {
    onsignal: Option<Callback<Context>>,
    job: Box<Task>,
    canvas: Option<CanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
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
                if self.canvas.is_none() {
                    match document().query_selector("#canvas") {
                        Ok(Some(canvas)) => {
                            let canvas: CanvasElement = canvas.try_into().unwrap();
                            let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
                            self.canvas = Some(canvas);
                            self.ctx = Some(ctx);
                            context.console.log("found canvas");
                            ()
                        },
                        _ => (),
                    }
                }
                match (&self.canvas, &self.ctx) {
                    (Some(canvas), Some(ctx)) => {
                        ctx.move_to(0.0, 0.0);
                        ctx.line_to(800.0, 600.0);
                        ctx.stroke();
                    }
                    _ => {}
                }
                self.job = GameModel::animate(context);
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties, context: &mut Env<Context, Self>) -> bool {
        context.console.log("updating game model props");
        false
    }
}

impl Renderable<Context, GameModel> for GameModel {
    fn view(&self) -> Html<Context, GameModel> {
        html! {
          <canvas id="canvas", width=600, height=400,></canvas>
        }
    }
}

impl GameModel {
    fn animate(context: &mut Env<Context, Self>) -> Box<Task> {
        let send_back = context.send_back(|_| GameMessage::Animate);
        Box::new(context.timeout.spawn(Duration::from_millis(1000 / 60 as u64), send_back))
    }
}