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
    layer: usize,
    z_order: usize
}

impl GameObject {
    pub fn from_pixel(x: f64, y: f64, scale_x: f32, scale_y: f32, color: [u8; 3], color_idx: usize, scale_multi: f32) -> Self {
        Self {
            id: 211,
            x: (x + (scale_x - 1.) as f64 / 2.) * 30. * scale_multi as f64,
            y: (0. - y - (scale_y - 1.) as f64 / 2.) * 30. * scale_multi as f64,
            scale_x: scale_x * scale_multi,
            scale_y: scale_y * scale_multi,
            color_id: 1011,
            hsv: Hsv::rgb_to_hsv(color),
            layer: color_idx,
            z_order: color_idx
        }
    }
}