use nalgebra::*;

pub struct Camera {
    pub x: f32,
}

impl Camera {
    pub fn matrix(&self) -> Matrix4<f32> {
        let mut view_matrix = Matrix4::new_translation(&Vector3::new(self.x, 0.0, -6.0));
        view_matrix *= Matrix4::from_euler_angles(std::f32::consts::PI / 12.0, 0.0, 0.0);
        view_matrix
    }

    pub fn interpolate(&mut self, to: f32, delta: f64) {
        self.x += (to - self.x) * (delta as f32);
    }
}
