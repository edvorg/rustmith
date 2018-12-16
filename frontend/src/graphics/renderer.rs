use crate::fps::FpsModel;
use crate::fps::FpsStats;
use crate::graphics::objects::make_fret;
use crate::graphics::objects::make_object;
use crate::graphics::objects::Object;
use crate::graphics::shaders::make_program;
use crate::graphics::shaders::Program;
use crate::registry::Registry;
use crate::services::ext::CanvasElementExt;
use crate::services::ext::WebGLRenderingContextExt;
use nalgebra::*;
use rustmith_common::ext::DurationExt;
use rustmith_common::track::Action;
use rustmith_common::track::Fret;
use rustmith_common::track::Track;
use std::time::Duration;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::event::ResizeEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::window;
use stdweb::web::IEventTarget;
use stdweb::web::IParentNode;
use stdweb::web::RequestAnimationFrameHandle;
use webgl_rendering_context::WebGLRenderingContext as gl;
use yew::prelude::Component;
use yew::prelude::Env;
use yew::prelude::Html;
use yew::prelude::Renderable;

pub struct Renderer {
    pub program: Program,
    pub view_matrix: Matrix4<f32>,
    pub frets: Vec<Object>,
    pub context: gl,
    pub width: f32,
    pub height: f32,
}

pub struct RendererModel {
    renderer: Option<Renderer>,
    job: Box<RequestAnimationFrameHandle>,
    last_time: Option<f64>,
    game_time: f64,
    fps: FpsStats,
    fps_snapshot: FpsStats,
    track: Option<Track>,
}

pub enum RendererMessage {
    Animate { time: f64 },
    Resize((f32, f32)),
}

#[derive(Clone, PartialEq)]
pub struct RendererProps {
    pub track: Option<Track>,
}

impl Default for RendererProps {
    fn default() -> Self {
        RendererProps { track: None }
    }
}

impl Component<Registry> for RendererModel {
    type Message = RendererMessage;
    type Properties = RendererProps;

    fn create(props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        RendererModel {
            renderer: None,
            last_time: None,
            game_time: 0.0,
            job: RendererModel::animate(env),
            fps: FpsStats::new(),
            fps_snapshot: FpsStats::new(),
            track: props.track,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        match msg {
            RendererMessage::Animate { time } => {
                if self.renderer.is_none() {
                    self.renderer = self.setup_graphics(env);
                }
                let delta_millis = time - self.last_time.unwrap_or(time);
                if let (Some(r), Some(track)) = (&mut self.renderer, &self.track) {
                    let track_view = track.view(Duration::from_millis(self.game_time as u64));
                    r.render(self.game_time, track_view);
                } else {
                    env.console.warn("Something is wrong, renderer not found");
                }
                self.job = RendererModel::animate(env);
                self.last_time = Some(time);
                self.game_time += delta_millis;

                self.fps.log_frame(delta_millis);
                if self.fps.time > 2000.0 {
                    self.fps.drain(&mut self.fps_snapshot);
                    true
                } else {
                    false
                }
            }

            RendererMessage::Resize((width, height)) => {
                env.console.log(&format!("Canvas resized ({}, {})", width, height));
                if let Some(r) = &mut self.renderer {
                    r.set_viewport(width, height);
                } else {
                    env.console.warn("Something is wrong, renderer not found");
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _env: &mut Env<Registry, Self>) -> bool {
        self.track = props.track;
        false
    }
}

impl Renderer {
    pub fn render(&mut self, game_time: f64, track_view: Vec<&Action>) {
        self.context.enable(gl::DEPTH_TEST);
        self.context.depth_func(gl::LEQUAL);
        self.context.clear_color(0.5, 0.5, 0.5, 0.9);
        self.context.clear_depth(1.0);
        self.context.viewport(0, 0, self.width as i32, self.height as i32);

        self.context.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        let proj_matrix = Matrix4::new_perspective(self.width / self.height, 60.0 / 180.0 * std::f32::consts::PI, 1.0, 1000.0);
        self.context
            .uniform_matrix4fv(Some(&self.program.proj_matrix_location), false, &proj_matrix.as_slice()[..]);

        self.context
            .uniform_matrix4fv(Some(&self.program.view_matrix_location), false, &self.view_matrix.as_slice()[..]);

        for action in track_view {
            match action {
                Action::Fret(Fret { fret, string, .. }) => {
                    // Position
                    self.context
                        .bind_buffer(gl::ARRAY_BUFFER, Some(&self.frets[*string as usize - 1].vertex_buffer));
                    self.context.vertex_attrib_pointer(self.program.position, 3, gl::FLOAT, false, 0, 0);
                    self.context.enable_vertex_attrib_array(self.program.position);

                    // Color
                    self.context
                        .bind_buffer(gl::ARRAY_BUFFER, Some(&self.frets[*string as usize - 1].color_buffer));
                    self.context.vertex_attrib_pointer(self.program.color, 3, gl::FLOAT, false, 0, 0);
                    self.context.enable_vertex_attrib_array(self.program.color);

                    // Indices
                    self.context
                        .bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.frets[*string as usize - 1].index_buffer));

                    let mut model_matrix: Matrix4<f32> = Matrix4::identity();
                    let x = f32::from(*fret);
                    let y = f32::from(*string) - 5.0;
                    let z = (action.starts_at().total_millis() as f32 - game_time as f32) * 0.001 as f32;
                    let length = (action.ends_at().total_millis() - action.starts_at().total_millis()) as f32 * 0.001;
                    model_matrix *= Matrix4::new_translation(&Vector3::new(x, y, -z));
                    model_matrix *= Matrix4::from_diagonal(&Vector4::new(0.5, 0.25, 0.5 * length, 1.0));
                    model_matrix *= Matrix4::new_translation(&Vector3::new(0.0, 0.0, -1.0));
                    self.context
                        .uniform_matrix4fv(Some(&self.program.model_matrix_location), false, &model_matrix.as_slice()[..]);
                    self.context.draw_elements(gl::TRIANGLES, 36, gl::UNSIGNED_SHORT, 0);
                }
                _ => {
                    js! { console.log("unsupported action"); };
                }
            }
        }
    }

    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.context.update_size((width, height));
        self.width = width;
        self.height = height;
    }

    pub fn new(context: gl, size: (f32, f32)) -> Self {
        context.update_size(size);
        let (width, height) = size;

        let program = make_program(&context);

        let frets = vec![
            make_object(&context, make_fret(223.0 / 255.0, 105.0 / 255.0, 250.0 / 255.0)),
            make_object(&context, make_fret(97.0 / 255.0, 246.0 / 255.0, 35.0 / 255.0)),
            make_object(&context, make_fret(245.0 / 255.0, 167.0 / 255.0, 25.0 / 255.0)),
            make_object(&context, make_fret(50.0 / 255.0, 216.0 / 255.0, 228.0 / 255.0)),
            make_object(&context, make_fret(220.0 / 255.0, 217.0 / 255.0, 49.0 / 255.0)),
            make_object(&context, make_fret(226.0 / 255.0, 47.0 / 255.0, 44.0 / 255.0)),
        ];

        let mut view_matrix = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -10.0));
        view_matrix *= Matrix4::from_euler_angles(std::f32::consts::PI / 6.0, 0.0, 0.0);

        Renderer {
            program,
            view_matrix,
            frets,
            context,
            width,
            height,
        }
    }
}

impl Renderable<Registry, RendererModel> for RendererModel {
    fn view(&self) -> Html<Registry, RendererModel> {
        html! {
          <>
            <FpsModel: fps=&self.fps_snapshot, />
            <canvas id="canvas",></canvas>
          </>
        }
    }
}

impl RendererModel {
    fn animate(env: &mut Env<Registry, Self>) -> Box<RequestAnimationFrameHandle> {
        let send_back = env.send_back(|time| RendererMessage::Animate { time });
        let f = move |d| {
            send_back.emit(d);
        };
        Box::new(window().request_animation_frame(f))
    }

    fn setup_graphics(&self, env: &mut Env<Registry, Self>) -> Option<Renderer> {
        env.console.log("Setting up graphics context");
        match document().query_selector("#canvas") {
            Ok(Some(canvas)) => {
                let mut canvas: CanvasElement = canvas.try_into().unwrap();
                let context = canvas.make_context();
                let renderer = Renderer::new(context, canvas.adjust_dpi());
                let callback = env.send_back(|m| m);
                window().add_event_listener(move |_: ResizeEvent| {
                    callback.emit(RendererMessage::Resize(canvas.adjust_dpi()));
                });
                env.console.log("Graphics context inititalized");
                Some(renderer)
            }
            _ => None,
        }
    }
}
