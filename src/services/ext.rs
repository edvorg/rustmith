use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;
use webgl_rendering_context::WebGLRenderingContext;
use stdweb::web::Window;
use yew_audio::MediaStreamSource;
use yew_audio::AudioNode;

pub trait WindowExt {
    fn device_pixel_ratio(&self) -> f64;
    fn set_source(&self, source: &MediaStreamSource);
}

impl WindowExt for Window {
    fn device_pixel_ratio(&self) -> f64 {
        js! (
          return @{self}.devicePixelRatio;
        ).try_into().unwrap()
    }

    fn set_source(&self, source: &MediaStreamSource) {
        js! {
            @{self}.source = @{&source.js()};
        }
    }
}

pub trait CanvasElementExt {
    fn client_width(&self) -> f64;
    fn client_height(&self) -> f64;
}

impl CanvasElementExt for CanvasElement {
    fn client_width(&self) -> f64 {
        js! (
            return @{self}.clientWidth;
        ).try_into().unwrap()
    }

    fn client_height(&self) -> f64 {
        js! (
            return @{self}.clientHeight;
        ).try_into().unwrap()
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
