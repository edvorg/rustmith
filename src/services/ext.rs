use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;
use webgl_rendering_context::WebGLRenderingContext;

pub trait HiDPI {
    fn device_pixel_ratio() -> f64;
    fn client_width(&self) -> f64;
    fn client_height(&self) -> f64;
}

impl HiDPI for CanvasElement {
    fn device_pixel_ratio() -> f64 {
        js! (
          return window.devicePixelRatio;
        ).try_into().unwrap()
    }

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

pub trait Viewport {
    fn set_viewport_width(&self, width: f32);
    fn set_viewport_height(&self, height: f32);
    fn update_size(&self, size: (f32, f32));
}

impl Viewport for WebGLRenderingContext {
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