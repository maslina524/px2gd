use std::{collections::HashMap, path::Path};
use image::{ImageBuffer, ImageReader, Rgba};
use bitvec::{bitvec, vec::BitVec};

use crate::object::{GameObject};

pub fn run(file: &Path, x_offset: f64, y_offset: f64, scale_multi: f32, const_scale: Option<f32>, step: u8, no_optimization: bool) -> Result<Vec<GameObject>, Box<dyn std::error::Error>> {
    let dyn_img = ImageReader::open(file)?.decode()?; // open image
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = dyn_img.to_rgba8();
    let (w, h) = (img.width(), img.height());
    let total_pixels = w * h;

    let scale_multi = if let Some(scale) = const_scale {
        let max = w.max(h);
        (scale / 30.0) / max as f32
    } else {
        scale_multi
    };

    let colors = {
        let mut occupied = bitvec![0; total_pixels as usize];
        get_color_frequency(&img, step, w, h, &mut occupied)
    };

    let mut objects = Vec::new();

    if no_optimization {
        for (color_idx, &color) in colors.iter().enumerate() {
            let mut temp_layer = bitvec![0; total_pixels as usize];
            let mut y = 0;
            while y < h as usize {
                let mut x = 0;
                while x < w as usize {
                    let idx = y * w as usize + x;
                    if !temp_layer[idx] {
                        let pixel = img.get_pixel(x as u32, y as u32).0;
                        let quantized = quantize_rgba(pixel, step);
                        if quantized == color {
                            let (scale_x, scale_y) = find_max_rectangle(&temp_layer, &img, w, h, x, y, color, step);
                            // Mark rectangle in temp_layer
                            for yy in y..y + scale_y as usize {
                                let base = yy * w as usize;
                                for xx in x..x + scale_x as usize {
                                    temp_layer.set(base + xx, true);
                                }
                            }
                            let mut obj = GameObject::from_pixel(
                                x as u32, 
                                y as u32, 
                                scale_x, 
                                scale_y, 
                                color[..3].try_into().unwrap(), 
                                color_idx, 
                                scale_multi
                            );
                            obj.x += x_offset;
                            obj.y += y_offset;
                            objects.push(obj);
                        }
                    }
                    x += 1;
                }
                y += 1;
            }
        }
    } else {
        let mut occupied = bitvec![0; total_pixels as usize];
        let _ = get_color_frequency(&img, step, w, h, &mut occupied);

        for (color_idx, &color) in colors.iter().enumerate() {
            let mut temp_layer = bitvec![0; total_pixels as usize];
            let mut y = 0;
            while y < h as usize {
                let mut x = 0;
                while x < w as usize {
                    let idx = y * w as usize + x;
                    if !occupied[idx] && !temp_layer[idx] {
                        let pixel = img.get_pixel(x as u32, y as u32).0;
                        let quantized = quantize_rgba(pixel, step);
                        if quantized == color {
                            let (scale_x, scale_y) = find_max_rectangle(&occupied, &img, w, h, x, y, color, step);
                            // Mark rectangle in temp_layer
                            for yy in y..y + scale_y as usize {
                                let base = yy * w as usize;
                                for xx in x..x + scale_x as usize {
                                    temp_layer.set(base + xx, true);
                                }
                            }
                            let mut obj = GameObject::from_pixel(
                                x as u32, 
                                y as u32, 
                                scale_x, 
                                scale_y, 
                                color[..3].try_into().unwrap(), 
                                color_idx, 
                                scale_multi
                            );
                            obj.x += x_offset;
                            obj.y += y_offset;
                            objects.push(obj);
                        }
                    }
                    x += 1;
                }
                y += 1;
            }

            // Occupy all pixels of this color in the global occupied map
            for y in 0..h {
                for x in 0..w {
                    let idx = (y * w + x) as usize;
                    let pixel = img.get_pixel(x, y).0;
                    if quantize_rgba(pixel, step) == color {
                        occupied.set(idx, true);
                    }
                }
            }
        }
    }
    
    Ok(objects)
}

fn quantize_rgba(pixel: [u8; 4], step: u8) -> [u8; 4] {
    pixel.map(|v| (v / step) * step)
}

fn get_color_frequency(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    step: u8,
    width: u32,
    height: u32,
    occupied: &mut BitVec,
) -> Vec<[u8; 4]> {
    let mut map = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let pixel = img.get_pixel(x, y).0;
            let quantized = quantize_rgba(pixel, step);
            if quantized[3] != 0 {
                *map.entry(quantized).or_insert(0u32) += 1;
            } else {
                occupied.set(idx, true);
            }
        }
    }
    let mut sorted: Vec<(&[u8; 4], &u32)> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    sorted.into_iter().map(|(c, _)| *c).collect()
}

fn find_max_rectangle(
    occupied: &BitVec,
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    x: usize,
    y: usize,
    target_color: [u8; 4],
    step: u8,
) -> (u32, u32) {
    let mut scale_x = 1;
    let mut scale_y = 1;

    // Expand to the right - проверяем и цвет, и occupied
    while x + scale_x < width as usize {
        let check_idx = y * width as usize + (x + scale_x);
        if occupied[check_idx] {
            break;
        }
        let pixel = img.get_pixel((x + scale_x) as u32, y as u32).0;
        let quantized = quantize_rgba(pixel, step);
        if quantized != target_color {
            break;
        }
        scale_x += 1;
    }

    // Expand downward row by row - проверяем и цвет, и occupied
    while y + scale_y < height as usize {
        let mut row_ok = true;
        for xx in x..x + scale_x {
            let check_idx = (y + scale_y) * width as usize + xx;
            if occupied[check_idx] {
                row_ok = false;
                break;
            }
            let pixel = img.get_pixel(xx as u32, (y + scale_y) as u32).0;
            let quantized = quantize_rgba(pixel, step);
            if quantized != target_color {
                row_ok = false;
                break;
            }
        }
        if !row_ok {
            break;
        }
        scale_y += 1;
    }

    (scale_x as u32, scale_y as u32)
}