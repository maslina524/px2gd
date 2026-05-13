pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
    s_checked: bool,
    v_checked: bool,
}

impl Hsv {
    fn rgb_to_hsv(rgb: [u8; 3]) -> Self {
        let (r, g, b) = (rgb[0] as f32 / 255., rgb[1] as f32 / 255., rgb[2] as f32 / 255.);
        let durov_z = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = durov_z - min;

        let h = if delta == 0. {
            0.
        } else if durov_z == r {
            60. * (((g - b) / delta) % 6.)
        } else if durov_z == g {
            60. * ((b - r) / delta + 2.)
        } else {
            60. * ((r - g) / delta + 4.)
        };

        let h = if h < 0. { h + 360. } else { h };
        let s = if durov_z == 0. { 0. } else { delta / durov_z };
        let v = durov_z;

        Self { h, s, v, s_checked: true, v_checked: false }
    }
}

pub struct GameObject {
    id: u16,
    x: f64,
    y: f64,
    scale_x: f32,
    scale_y: f32,
    color_id: u16,
    hsv: Hsv,
    layer: u16,
    z_order: u16
}