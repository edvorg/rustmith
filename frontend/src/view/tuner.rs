use crate::model::tuner::*;
use yew::prelude::*;
use crate::registry::Registry;

impl Renderable<Registry, TunerModel> for TunerModel {
    fn view(&self) -> Html<Registry, TunerModel> {
        match &self.note {
            Some(n) => {
                let note_message = format!("Note: {}", n.name);
                let note_frequency = format!("Frequency: {}hz", n.frequency);
                html! {
                    <div id="game-tuner",>
                      <button id="note-button", onclick = |_| TunerMessage::ToggleE ,> { "Play E" } </button>
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
}
