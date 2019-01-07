use crate::model::editor::*;
use yew::prelude::*;
use crate::registry::Registry;

impl Renderable<Registry, EditorModel> for EditorModel {
    fn view(&self) -> Html<Registry, EditorModel> {
        html! {
          <div class="editor",>
            <div class="editor-controls",>
              <div>
              <input id="songNameInput",
                     type="text",
                     placeholder="Name",
                     oninput=|e| EditorMessage::UpdateSongName(e.value),></input>
              <input id="songUrlInput",
                     type="text",
                     placeholder="Youtube Id",
                     oninput=|e| EditorMessage::UpdateSongYoutubeId(e.value),></input>
              <textarea value=self.song_content.as_ref().unwrap_or(&String::from("")).clone(),
                        id="songUrlInput",
                        type="text",
                        placeholder="Data",
                        oninput=|e| EditorMessage::UpdateSongContent(e.value),></textarea>
              </div>
              <button onclick=|_| EditorMessage::SaveAndExit,>
                { "Save and exit" }
              </button>
              <button onclick=|_| EditorMessage::Route(RoutingMessage::Exit),>
                { "Discard and exit" }
              </button>
              <button onclick=|_| EditorMessage::StartRecording,>
                { "Start recording" }
              </button>
            </div>
            { self.video_view() }
            { self.fretboard_view() }
          </div>
        }
    }
}

impl EditorModel {
    fn fret_class(&self, string: u8, _fret: u8) -> &'static str {
        match string {
            0 => "string-0",
            1 => "string-1",
            2 => "string-2",
            3 => "string-3",
            4 => "string-4",
            5 => "string-5",
            6 => "string-6",
            _ => "",
        }
    }

    fn fret(&self, string: u8, fret: u8) -> Html<Registry, EditorModel> {
        let class = self.fret_class(string, fret);
        html! {
          <td class=class,
              onmousedown=|_| EditorMessage::SetPosition { string, fret },
              onmouseup=|_| EditorMessage::LogFret,>
            { format!("{}", fret + 1) }
          </td>
        }
    }

    fn string_row(&self, string: u8) -> Html<Registry, EditorModel> {
        html! {
          <tr>
            { for (0..24).map(|fret| self.fret(string, fret)) }
          </tr>
        }
    }

    fn video_view(&self) -> Html<Registry, EditorModel> {
        if self.recording {
            html! {
              <div class="editor-video",>
                <iframe id="editor-clip",
                        src=&self.song_url.clone().unwrap(),
                        frameborder="0",
                        allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture",>
                </iframe>
              </div>
            }
        } else {
            html! {
              <div class="editor-video",>
                {  "video" }
              </div>
            }
        }
    }

    fn fretboard_view(&self) -> Html<Registry, EditorModel> {
        html! {
          <div class="editor-fretboard",>
            <table width="100%",>
              { for (0..7).map(|string| self.string_row(string)) }
            </table>
          </div>
        }
    }
}
