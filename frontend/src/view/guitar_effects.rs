use crate::model::guitar_effects::*;
use yew::prelude::*;
use crate::registry::Registry;

impl Renderable<Registry, GuitarEffectsModel> for GuitarEffectsModel {
    fn view(&self) -> Html<Registry, GuitarEffectsModel> {
        html! {
            <div class="game-effects",>
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
