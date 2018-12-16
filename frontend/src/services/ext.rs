use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::window;
use stdweb::web::Window;
use webgl_rendering_context::WebGLRenderingContext;
use yew_audio::AudioNode;
use yew_audio::MediaStreamSource;

pub trait WindowExt {
    fn set_source(&self, source: &MediaStreamSource);
}

impl WindowExt for Window {
    fn set_source(&self, source: &MediaStreamSource) {
        js! {
            @{self}.source = @{&source.js()};
        }
    }
}

pub trait CanvasElementExt {
    fn client_width(&self) -> f64;
    fn client_height(&self) -> f64;
    fn size(&self) -> (f32, f32);
    fn adjust_dpi(&mut self) -> (f32, f32);
    fn make_context(&self) -> WebGLRenderingContext;
}

impl CanvasElementExt for CanvasElement {
    fn client_width(&self) -> f64 {
        js! (
            return @{self}.clientWidth;
        )
        .try_into()
        .unwrap()
    }

    fn client_height(&self) -> f64 {
        js! (
            return @{self}.clientHeight;
        )
        .try_into()
        .unwrap()
    }

    fn size(&self) -> (f32, f32) {
        (self.width() as f32, self.height() as f32)
    }

    fn adjust_dpi(&mut self) -> (f32, f32) {
        let real_to_css_pixels = window().device_pixel_ratio();
        let display_width = (self.client_width() * real_to_css_pixels).floor() as u32;
        let display_height = (self.client_height() * real_to_css_pixels).floor() as u32;
        if self.width() != display_width || self.height() != display_height {
            self.set_width(display_width);
            self.set_height(display_height);
        }
        self.size()
    }

    fn make_context(&self) -> WebGLRenderingContext {
        self.get_context().unwrap()
    }
}

pub trait WebGLRenderingContextExt {
    fn set_viewport_width(&self, width: f32);
    fn set_viewport_height(&self, height: f32);
    fn update_size(&self, size: (f32, f32));
}

impl WebGLRenderingContextExt for WebGLRenderingContext {
    fn set_viewport_width(&self, width: f32) {
        js! (
            @{self}.viewportWidth = @{width};
        );
    }

    fn set_viewport_height(&self, height: f32) {
        js! (
            @{self}.viewportHeight = @{height};
        );
    }

    fn update_size(&self, size: (f32, f32)) {
        let (width, height) = size;
        self.set_viewport_width(width);
        self.set_viewport_height(height);
    }
}
