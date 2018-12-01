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