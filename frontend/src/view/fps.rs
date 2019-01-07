use crate::model::fps::*;
use yew::prelude::*;
use crate::registry::Registry;

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
