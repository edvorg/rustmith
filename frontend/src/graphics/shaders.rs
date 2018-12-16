use webgl_rendering_context::WebGLProgram;
use webgl_rendering_context::WebGLRenderingContext as gl;
use webgl_rendering_context::WebGLShader;
use webgl_rendering_context::WebGLUniformLocation;

pub static VERTEX_CODE: &str = r#"
        attribute vec3 position;
        uniform mat4 Pmatrix;
        uniform mat4 Vmatrix;
        uniform mat4 Mmatrix;
        attribute vec3 color;
        varying vec3 vColor;
        void main() {
            gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
            vColor = color;
        }
    "#;

pub static FRAGMENT_CODE: &str = r#"
        precision mediump float;
        varying vec3 vColor;
        void main() {
            gl_FragColor = vec4(vColor, 1.);
        }
    "#;

pub struct Program {
    pub vert_shader: WebGLShader,
    pub frag_shader: WebGLShader,
    pub shader_program: WebGLProgram,
    pub proj_matrix_location: WebGLUniformLocation,
    pub view_matrix_location: WebGLUniformLocation,
    pub model_matrix_location: WebGLUniformLocation,
    pub position: u32,
    pub color: u32,
}

pub fn make_program(context: &gl) -> Program {
    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&vert_shader, VERTEX_CODE);
    context.compile_shader(&vert_shader);

    let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&frag_shader, FRAGMENT_CODE);
    context.compile_shader(&frag_shader);

    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vert_shader);
    context.attach_shader(&shader_program, &frag_shader);
    context.link_program(&shader_program);
    context.use_program(Some(&shader_program));

    let proj_matrix_location = context.get_uniform_location(&shader_program, "Pmatrix").unwrap();
    let view_matrix_location = context.get_uniform_location(&shader_program, "Vmatrix").unwrap();
    let model_matrix_location = context.get_uniform_location(&shader_program, "Mmatrix").unwrap();

    let position = context.get_attrib_location(&shader_program, "position") as u32;
    let color = context.get_attrib_location(&shader_program, "color") as u32;

    Program {
        vert_shader,
        frag_shader,
        shader_program,
        proj_matrix_location,
        view_matrix_location,
        model_matrix_location,
        position,
        color,
    }
}
