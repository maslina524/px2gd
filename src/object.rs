pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
    s_checked: bool,
    v_checked: bool,
}

pub struct GameObject {
    x: f64,
    y: f64,
    scale_x: f32,
    scale_y: f32,
    hsv: Hsv,
    layer: u16,
    z_order: u16
}