use image::{ImageBuffer, Rgb, RgbImage};
use crate::fsaa::FSAABuffer;

pub fn draw_circle_no_aa(width: u32, height: u32) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = height as f32 / 3.0;
    
    for y in 0..height {
        for x in 0..width {
            let pixel_center_x = x as f32 + 0.5;
            let pixel_center_y = y as f32 + 0.5;
            
            let dx = pixel_center_x - center_x;
            let dy = pixel_center_y - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance <= radius {
                img.put_pixel(x, y, Rgb([255, 0, 0]));
            } else {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }
    
    img
}

pub fn draw_circle_fsaa(width: u32, height: u32, scale_factor: u32) -> RgbImage {
    let mut buffer = FSAABuffer::new(width, height, scale_factor);
    
    let high_res_width = width * scale_factor;
    let high_res_height = height * scale_factor;
    let center_x = high_res_width as f32 / 2.0;
    let center_y = high_res_height as f32 / 2.0;
    let radius = high_res_height as f32 / 3.0;
    
    for y in 0..high_res_height {
        for x in 0..high_res_width {
            let pixel_center_x = x as f32 + 0.5;
            let pixel_center_y = y as f32 + 0.5;
            
            let dx = pixel_center_x - center_x;
            let dy = pixel_center_y - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance <= radius {
                buffer.set_pixel(x, y, Rgb([255, 0, 0]));
            } else {
                buffer.set_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }
    
    buffer.resolve()
}