use image::{ImageBuffer, Rgb, RgbImage};

pub struct FSAABuffer {
    width: u32,
    height: u32,
    scale_factor: u32,
    buffer: RgbImage,
}

impl FSAABuffer {
    pub fn new(width: u32, height: u32, scale_factor: u32) -> Self {
        let high_res_width = width * scale_factor;
        let high_res_height = height * scale_factor;
        let buffer = ImageBuffer::new(high_res_width, high_res_height);
        
        FSAABuffer {
            width,
            height,
            scale_factor,
            buffer,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb<u8>) {
        if x < self.width * self.scale_factor && y < self.height * self.scale_factor {
            self.buffer.put_pixel(x, y, color);
        }
    }

    pub fn resolve(&self) -> RgbImage {
        let mut result = ImageBuffer::new(self.width, self.height);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                
                for sy in 0..self.scale_factor {
                    for sx in 0..self.scale_factor {
                        let high_res_x = x * self.scale_factor + sx;
                        let high_res_y = y * self.scale_factor + sy;
                        let pixel = self.buffer.get_pixel(high_res_x, high_res_y);
                        
                        r_sum += pixel[0] as u32;
                        g_sum += pixel[1] as u32;
                        b_sum += pixel[2] as u32;
                    }
                }
                
                let sample_count = (self.scale_factor * self.scale_factor) as u32;
                let r = (r_sum / sample_count) as u8;
                let g = (g_sum / sample_count) as u8;
                let b = (b_sum / sample_count) as u8;
                
                result.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        result
    }
}