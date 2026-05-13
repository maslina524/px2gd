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
            self.s_checked as u8, self.v_checked as u8
        )
    }
}

pub struct GameObject {
    pub id: u16,
    pub x: f64,
    pub y: f64,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color_id: u16,
    pub hsv: Hsv,
    pub layer: usize,
    pub z_order: usize
}

impl GameObject {
    pub fn from_pixel(x: u32, y: u32, size_x: u32, size_y: u32, color: [u8; 3], color_idx: usize, scale_multi: f32) -> Self {
        let x_pos = (x as f64 + size_x as f64 / 2.0) * 30.0;
        let y_pos = (y as f64 + size_y as f64 / 2.0) * 30.0;

        Self {
            id: 211,
            x: x_pos * scale_multi as f64,
            y: -y_pos * scale_multi as f64,
            scale_x: size_x as f32 * scale_multi,
            scale_y: size_y as f32 * scale_multi,
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