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

impl ToString for Hsv {
    fn to_string(&self) -> String {
        format!(
            "{}a{}a{}a{}a{}",
            self.h, self.s, self.v,
            self.s_checked, self.v_checked
        )
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
    pub fn from_pixel(x: u32, y: u32, scale_x: u32, scale_y: u32, color: [u8; 3], color_idx: usize, scale_multi: f32) -> Self {
        Self {
            id: 211,
            x: (x as f64 + (scale_x - 1) as f64 / 2.) * 30. * scale_multi as f64,
            y: (0. - y as f64 - (scale_y - 1) as f64 / 2.) * 30. * scale_multi as f64,
            scale_x: scale_x as f32 * scale_multi,
            scale_y: scale_y as f32 * scale_multi,
            color_id: 1011,
            hsv: Hsv::rgb_to_hsv(color),
            layer: color_idx,
            z_order: color_idx
        }
    }
}

impl ToString for GameObject {
    fn to_string(&self) -> String {
        let mut parts = Vec::with_capacity(9);

        parts.push(format!("1,{}", self.id));
        parts.push(format!("2,{}", self.x));
        parts.push(format!("3,{}", self.y));
        parts.push(format!("128,{}", self.scale_x));
        parts.push(format!("129,{}", self.scale_y));
        parts.push(format!("21,{}", self.color_id));
        parts.push(format!("41,1,43,{}", self.hsv.to_string()));
        parts.push(format!("20,{}", self.layer));
        parts.push(format!("25,{}", self.z_order));

        let mut ret = parts.join(",");
        ret.push(';');
        ret
    }
}