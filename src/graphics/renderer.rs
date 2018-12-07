use crate::graphics::algebra;
use crate::graphics::shaders;
use stdweb::web::{
    TypedArray,
};
use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLUniformLocation,
    WebGLBuffer
};
use stdweb::web::html_element::CanvasElement;
use crate::services::ext::Viewport;

pub struct Renderer {
    pub p_matrix: WebGLUniformLocation,
    pub v_matrix: WebGLUniformLocation,
    pub m_matrix: WebGLUniformLocation,
    pub mov_matrix: [f32; 16],
    pub view_matrix: [f32; 16],
    pub index_buffer: WebGLBuffer,
    pub context: gl,
    pub width: f32,
    pub height: f32,
}

impl Renderer {
    pub fn render(&mut self, delta: f64) {
        self.context.clear_color(1.0, 0.0, 0.0, 1.0);
        self.context.clear(gl::COLOR_BUFFER_BIT);
        algebra::rotate_z(&mut self.mov_matrix, delta as f32);
        algebra::rotate_y(&mut self.mov_matrix, delta as f32);
        algebra::rotate_x(&mut self.mov_matrix, delta as f32);
        self.context.enable(gl::DEPTH_TEST);
        self.context.depth_func(gl::LEQUAL);
        self.context.clear_color(0.5, 0.5, 0.5, 0.9);
        self.context.clear_depth(1.0);
        let proj_matrix = algebra::get_projection(40., self.width / self.height, 1., 100.);
        self.context.viewport(0, 0, self.width as i32, self.height as i32);
        self.context.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.context.uniform_matrix4fv(Some(&self.p_matrix), false, &proj_matrix[..]);
        self.context.uniform_matrix4fv(Some(&self.v_matrix), false, &self.view_matrix[..]);
        self.context.uniform_matrix4fv(Some(&self.m_matrix), false, &self.mov_matrix[..]);
        self.context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        self.context.draw_elements(gl::TRIANGLES, 36, gl::UNSIGNED_SHORT, 0);
    }

    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.context.update_size((width, height));
        self.width = width;
        self.height = height;
    }

    pub fn make_cotext(canvas: &CanvasElement) -> gl {
        canvas.get_context().unwrap()
    }

    pub fn new(context: gl, size: (f32, f32)) -> Self {
        context.update_size(size);
        let (width, height) = size;
        let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
        context.shader_source(&vert_shader, shaders::VERTEX_CODE);
        context.compile_shader(&vert_shader);

        let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
        context.shader_source(&frag_shader, shaders::FRAGMENT_CODE);
        context.compile_shader(&frag_shader);

        let shader_program = context.create_program().unwrap();
        context.attach_shader(&shader_program, &vert_shader);
        context.attach_shader(&shader_program, &frag_shader);
        context.link_program(&shader_program);
        context.use_program(Some(&shader_program));

        /* ====== Associating attributes to vertex shader =====*/
        let p_matrix = context.get_uniform_location(&shader_program, "Pmatrix").unwrap();
        let v_matrix = context.get_uniform_location(&shader_program, "Vmatrix").unwrap();
        let m_matrix = context.get_uniform_location(&shader_program, "Mmatrix").unwrap();

        let vertices = TypedArray::<f32>::from(&[
            -1.,-1.,-1.,  1.,-1.,-1.,  1., 1.,-1., -1., 1.,-1.,
            -1.,-1., 1.,  1.,-1., 1.,  1., 1., 1., -1., 1., 1.,
            -1.,-1.,-1., -1., 1.,-1., -1., 1., 1., -1.,-1., 1.,
            1.,-1.,-1.,  1., 1.,-1.,  1., 1., 1.,  1.,-1., 1.,
            -1.,-1.,-1., -1.,-1., 1.,  1.,-1., 1.,  1.,-1.,-1.,
            -1., 1.,-1., -1., 1., 1.,  1., 1., 1.,  1., 1.,-1.,
        ][..]).buffer();

        let colors = TypedArray::<f32>::from(&[
            5.,3.,7., 5.,3.,7., 5.,3.,7., 5.,3.,7.,
            1.,1.,3., 1.,1.,3., 1.,1.,3., 1.,1.,3.,
            0.,0.,1., 0.,0.,1., 0.,0.,1., 0.,0.,1.,
            1.,0.,0., 1.,0.,0., 1.,0.,0., 1.,0.,0.,
            1.,1.,0., 1.,1.,0., 1.,1.,0., 1.,1.,0.,
            0.,1.,0., 0.,1.,0., 0.,1.,0., 0.,1.,0.
        ][..]).buffer();

        let indices = TypedArray::<u16>::from(&[
            0,1,2, 0,2,3, 4,5,6, 4,6,7,
            8,9,10, 8,10,11, 12,13,14, 12,14,15,
            16,17,18, 16,18,19, 20,21,22, 20,22,23
        ][..]).buffer();

        // Create and store data into vertex buffer
        let vertex_buffer = context.create_buffer().unwrap();
        context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
        context.buffer_data_1(gl::ARRAY_BUFFER, Some(&vertices), gl::STATIC_DRAW);

        // Create and store data into color buffer
        let color_buffer = context.create_buffer().unwrap();
        context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
        context.buffer_data_1(gl::ARRAY_BUFFER, Some(&colors), gl::STATIC_DRAW);

        // Create and store data into index buffer
        let index_buffer = context.create_buffer().unwrap();
        context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

        context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
        let position = context.get_attrib_location(&shader_program, "position") as u32;
        context.vertex_attrib_pointer(position, 3, gl::FLOAT, false, 0, 0) ;

        // Position
        context.enable_vertex_attrib_array(position);
        context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
        let color = context.get_attrib_location(&shader_program, "color") as u32;
        context.vertex_attrib_pointer(color, 3, gl::FLOAT, false, 0, 0) ;

        // Color
        context.enable_vertex_attrib_array(color);

        let mov_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];
        let mut view_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];

        // translating z
        view_matrix[14] -= 6.;

        Renderer {
            p_matrix,
            v_matrix,
            m_matrix,
            mov_matrix,
            view_matrix,
            index_buffer,
            context,
            width,
            height,
        }
    }
}
