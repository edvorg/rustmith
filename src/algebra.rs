pub fn rotate_z(m: &mut [f32], angle: f32) {
    let (s, c) = angle.sin_cos();
    let (mv0, mv4, mv8) = (m[0], m[4], m[8]);

    m[0] = c*m[0]-s*m[1];
    m[4] = c*m[4]-s*m[5];
    m[8] = c*m[8]-s*m[9];

    m[1]=c*m[1]+s*mv0;
    m[5]=c*m[5]+s*mv4;
    m[9]=c*m[9]+s*mv8;
}

pub fn rotate_x(m: &mut [f32], angle: f32) {
    let (s, c) = angle.sin_cos();
    let (mv1, mv5, mv9) = (m[1], m[5], m[9]);

    m[1] = m[1]*c-m[2]*s;
    m[5] = m[5]*c-m[6]*s;
    m[9] = m[9]*c-m[10]*s;

    m[2] = m[2]*c+mv1*s;
    m[6] = m[6]*c+mv5*s;
    m[10] = m[10]*c+mv9*s;
}

pub fn rotate_y(m: &mut [f32], angle: f32) {
    let (s, c) = angle.sin_cos();
    let (mv0, mv4, mv8) = (m[0], m[4], m[8]);

    m[0] = c*m[0]+s*m[2];
    m[4] = c*m[4]+s*m[6];
    m[8] = c*m[8]+s*m[10];

    m[2] = c*m[2]-s*mv0;
    m[6] = c*m[6]-s*mv4;
    m[10] = c*m[10]-s*mv8;
}

pub fn get_projection(angle: f32, a: f32, z_min: f32, z_max: f32) -> [f32; 16] {
    let ang = (angle*0.5).to_radians().tan();
    return [
        0.5/ang, 0., 0., 0.,
        0., 0.5*a/ang, 0., 0.,
        0., 0., -(z_max+z_min)/(z_max-z_min), -1.,
        0., 0., (-2.*z_max*z_min)/(z_max-z_min), 0.
    ];
}