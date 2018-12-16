use stdweb::web::ArrayBuffer;
use stdweb::web::TypedArray;
use webgl_rendering_context::WebGLBuffer;
use webgl_rendering_context::WebGLRenderingContext as gl;

pub struct ObjectDesc {
    pub vertices: ArrayBuffer,
    pub indices: ArrayBuffer,
    pub colors: ArrayBuffer,
}

pub fn make_fret(r: f32, g: f32, b: f32) -> ObjectDesc {
    let vertices = TypedArray::<f32>::from(
        &[
            -1., -1., -1., 1., -1., -1., 1., 1., -1., -1., 1., -1., -1., -1., 1., 1., -1., 1., 1., 1., 1., -1., 1., 1., -1., -1., -1., -1., 1., -1.,
            -1., 1., 1., -1., -1., 1., 1., -1., -1., 1., 1., -1., 1., 1., 1., 1., -1., 1., -1., -1., -1., -1., -1., 1., 1., -1., 1., 1., -1., -1.,
            -1., 1., -1., -1., 1., 1., 1., 1., 1., 1., 1., -1.,
        ][..],
    )
    .buffer();

    let colors = TypedArray::<f32>::from(
        &[
            r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r,
            g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b, r, g, b,
        ][..],
    )
    .buffer();

    let indices = TypedArray::<u16>::from(
        &[
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
        ][..],
    )
    .buffer();

    ObjectDesc { vertices, indices, colors }
}

pub struct Object {
    pub desc: ObjectDesc,
    pub vertex_buffer: WebGLBuffer,
    pub index_buffer: WebGLBuffer,
    pub color_buffer: WebGLBuffer,
}

pub fn make_object(context: &gl, desc: ObjectDesc) -> Object {
    // Create and store data into vertex buffer
    let vertex_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&desc.vertices), gl::STATIC_DRAW);

    // Create and store data into color buffer
    let color_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&desc.colors), gl::STATIC_DRAW);

    // Create and store data into index buffer
    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&desc.indices), gl::STATIC_DRAW);

    Object {
        desc,
        vertex_buffer,
        index_buffer,
        color_buffer,
    }
}
