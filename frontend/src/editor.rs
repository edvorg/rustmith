use crate::registry::Registry;
use crate::services::track::make_youtube_url;
use crate::services::track::TrackCreateResult;
use crate::services::track::TrackService;
use stdweb::unstable::TryInto;
use yew::prelude::*;

fn now() -> f64 {
    js! (
      return performance.now();
    )
    .try_into()
    .unwrap()
}

pub enum RoutingMessage {
    Exit,
    ExitAndShowTrack(String),
}

pub enum EditorMessage {
    Route(RoutingMessage),
    UpdateSongName(String),
    UpdateSongYoutubeId(String),
    UpdateSongContent(String),
    SaveAndExit,
    Error(&'static str),
    SetPosition { string: u8, fret: u8 },
    LogFret,
    StartRecording,
}

pub struct EditorModel {
    pub onsignal: Option<Callback<RoutingMessage>>,
    pub song_name: Option<String>,
    pub song_youtube_id: Option<String>,
    pub song_content: Option<String>,
    pub current_string: u8,
    pub current_fret: u8,
    pub recording: bool,
    pub recording_from: u64,
    pub song_url: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct EditorProps {
    pub onsignal: Option<Callback<RoutingMessage>>,
}

impl Default for EditorProps {
    fn default() -> Self {
        EditorProps { onsignal: None }
    }
}

impl Component<Registry> for EditorModel {
    type Message = EditorMessage;
    type Properties = EditorProps;

    fn create(props: <Self as Component<Registry>>::Properties, _context: &mut Env<Registry, Self>) -> Self {
        EditorModel {
            onsignal: props.onsignal,
            song_name: None,
            song_youtube_id: None,
            song_content: None,
            current_string: 0,
            current_fret: 0,
            recording: false,
            recording_from: 0,
            song_url: None,
        }
    }

    fn update(&mut self, msg: <Self as Component<Registry>>::Message, context: &mut Env<Registry, Self>) -> bool {
        match msg {
            EditorMessage::StartRecording => {
                self.recording_from = now() as u64;
                self.recording = true;
                true
            }
            EditorMessage::SetPosition { string, fret } => {
                self.current_string = string;
                self.current_fret = fret;
                false
            }
            EditorMessage::LogFret => {
                let starts_at = now() as u64 - self.recording_from;
                let ends_at = starts_at + 100;
                let line = format!("fret:{}:{}:{}:{}", starts_at, ends_at, self.current_fret + 1, self.current_string + 1);
                if let Some(content) = &self.song_content {
                    self.song_content = Some(format!("{}\n{}", content, line));
                } else {
                    self.song_content = Some(line);
                }
                true
            }
            EditorMessage::Error(e) => {
                context.console.error(&format!("error {}", e));
                false
            }
            EditorMessage::Route(m) => {
                context.console.log(&format!("{:?} {:?}", &self.song_youtube_id, &self.song_name));
                if let Some(signal) = &self.onsignal {
                    signal.emit(m);
                }
                true
            }
            EditorMessage::SaveAndExit => match (&self.song_content, &self.song_name, &self.song_youtube_id) {
                (Some(content), Some(name), Some(youtube_id)) => {
                    let callback = context.send_back(|r: TrackCreateResult| match r {
                        TrackCreateResult::Created(id, _) => EditorMessage::Route(RoutingMessage::ExitAndShowTrack(id)),
                        TrackCreateResult::Error => EditorMessage::Error("create error"),
                    });
                    context.track.create_track(name, youtube_id, content, callback);
                    true
                }
                _ => {
                    context.console.error("Not all fields are set");
                    false
                }
            },
            EditorMessage::UpdateSongName(n) => {
                self.song_name = Some(n);
                true
            }
            EditorMessage::UpdateSongYoutubeId(n) => {
                self.song_url = Some(make_youtube_url(&n));
                self.song_youtube_id = Some(n);
                true
            }
            EditorMessage::UpdateSongContent(n) => {
                if n.is_empty() {
                    self.song_content = None;
                } else {
                    self.song_content = Some(n);
                }
                true
            }
        }
    }

    fn change(&mut self, props: <Self as Component<Registry>>::Properties, _context: &mut Env<Registry, Self>) -> bool {
        self.onsignal = props.onsignal;
        true
    }
}

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
