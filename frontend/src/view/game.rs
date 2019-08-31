use crate::graphics::renderer::RendererModel;
use crate::model::game::*;
use crate::model::guitar_effects::GuitarEffectsModel;
use crate::model::tuner::TunerModel;
use crate::registry::Registry;
use yew::prelude::*;

impl Renderable<Registry, GameModel> for GameModel {
    fn view(&self) -> Html<Registry, GameModel> {
        html! {
          <div class="game",>
            <div class="game-view",>
              <button id="exit-button", onclick = |_| GameMessage::Route(RoutingMessage::ExitGame),> { "exit" } </button>
              <RendererModel: track=&self.track, />
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
            <GuitarEffectsModel: mic=self.mic.clone(), />
            <TunerModel: mic=self.mic.clone(), />
          </div>
        }
    }
}
