use yew::prelude::Component;
use crate::registry::Registry;
use yew::prelude::Env;
use yew::prelude::Renderable;
use yew::prelude::Html;

#[derive(Clone, PartialEq)]
pub struct FpsStats {
    pub frames: u64,
    pub time: f64,
}

impl FpsStats {
    pub fn new() -> FpsStats {
        FpsStats {
            frames: 0,
            time: 0.0,
        }
    }

    pub fn log_frame(&mut self, frame_time: f64) {
        self.frames += 1;
        self.time += frame_time;
    }

    pub fn average_frame_time(&self) -> f64 {
        match self.frames {
            0 => 0.0,
            f => self.time / (f as f64)
        }

    }

    pub fn average_fps(&self) -> f64 {
        if self.time == 0.0 {
            0.0
        } else {
            (self.frames as f64) / self.time * 1000.0
        }
    }

    pub fn reset(&mut self) {
        self.frames = 0;
        self.time = 0.0;
    }

    pub fn drain(&mut self, to: &mut FpsStats) {
        to.reset();
        std::mem::swap(self, to);
    }
}

pub struct FpsModel {
    fps: FpsStats,
}

#[derive(Clone, PartialEq)]
pub struct FpsProps {
    pub fps: FpsStats,
}

impl Default for FpsProps {
    fn default() -> Self {
        FpsProps {
            fps: FpsStats::new(),
        }
    }
}

impl Component<Registry> for FpsModel {
    type Message = ();
    type Properties = FpsProps;

    fn create(props: <Self as Component<Registry>>::Properties, _env: &mut Env<Registry, Self>) -> Self {
        FpsModel {
            fps: props.fps,
        }
    }

    fn update(&mut self, _msg: Self::Message, _env: &mut Env<Registry, Self>) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties, _env: &mut Env<Registry, Self>) -> bool {
        self.fps = props.fps;
        true
    }
}

impl Renderable<Registry, FpsModel> for FpsModel {
    fn view(&self) -> Html<Registry, FpsModel> {
        html! {
          <div>
            <div id="fps",> { format!("avg. fps {}", &self.fps.average_fps()) } </div>
            <div id="delta",> { format!("avg. delta (ms) {}", &self.fps.average_frame_time()) } </div>
          </div>
        }
    }
}