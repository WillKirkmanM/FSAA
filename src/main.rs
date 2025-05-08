use image::{ImageBuffer, Rgb, RgbImage};

mod circle;
mod fsaa;

pub struct MultisampleBuffer {
    width: u32,
    height: u32,
    sample_count: u32,
    samples: Vec<Vec<Rgb<u8>>>,
}

impl MultisampleBuffer {
    pub fn new(width: u32, height: u32, sample_count: u32) -> Self {
        let samples = vec![vec![Rgb([0, 0, 0]); sample_count as usize]; (width * height) as usize];
        
        MultisampleBuffer {
            width,
            height,
            sample_count,
            samples,
        }
    }

    fn get_pixel_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn set_sample(&mut self, x: u32, y: u32, sample_index: u32, color: Rgb<u8>) {
        if x < self.width && y < self.height && sample_index < self.sample_count {
            let pixel_index = self.get_pixel_index(x, y);
            self.samples[pixel_index][sample_index as usize] = color;
        }
    }

    pub fn get_samples(&self, x: u32, y: u32) -> Option<&[Rgb<u8>]> {
        if x < self.width && y < self.height {
            let pixel_index = self.get_pixel_index(x, y);
            Some(&self.samples[pixel_index])
        } else {
            None
        }
    }

    pub fn resolve(&self) -> RgbImage {
        let mut img = ImageBuffer::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(samples) = self.get_samples(x, y) {
                    let mut r_sum = 0u32;
                    let mut g_sum = 0u32;
                    let mut b_sum = 0u32;

                    for sample in samples {
                        r_sum += sample[0] as u32;
                        g_sum += sample[1] as u32;
                        b_sum += sample[2] as u32;
                    }

                    let sample_count = samples.len() as u32;
                    let r = (r_sum / sample_count) as u8;
                    let g = (g_sum / sample_count) as u8;
                    let b = (b_sum / sample_count) as u8;

                    img.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
        }

        img
    }
}

pub fn generate_sample_positions(sample_count: u32) -> Vec<(f32, f32)> {
    match sample_count {
        1 => vec![(0.5, 0.5)],
        2 => vec![(0.25, 0.25), (0.75, 0.75)],
        4 => vec![
            (0.375, 0.125),
            (0.875, 0.375),
            (0.125, 0.625),
            (0.625, 0.875),
        ],
        8 => vec![
            (0.0625, 0.0625),
            (0.3125, 0.0625),
            (0.5625, 0.0625),
            (0.8125, 0.0625),
            (0.1875, 0.3125),
            (0.4375, 0.3125),
            (0.6875, 0.3125),
            (0.9375, 0.3125),
        ],
        _ => {
            let mut positions = Vec::with_capacity(sample_count as usize);
            let step = 1.0 / (sample_count as f32).sqrt();
            
            for i in 0..sample_count {
                let x = (i % (sample_count as f32).sqrt() as u32) as f32 * step + step / 2.0;
                let y = (i / (sample_count as f32).sqrt() as u32) as f32 * step + step / 2.0;
                positions.push((x, y));
            }
            
            positions
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multisample_buffer_creation() {
        let buffer = MultisampleBuffer::new(10, 10, 4);
        assert_eq!(buffer.width, 10);
        assert_eq!(buffer.height, 10);
        assert_eq!(buffer.sample_count, 4);
    }

    #[test]
    fn test_sample_setting_and_getting() {
        let mut buffer = MultisampleBuffer::new(10, 10, 4);
        buffer.set_sample(5, 5, 2, Rgb([255, 0, 0]));
        
        if let Some(samples) = buffer.get_samples(5, 5) {
            assert_eq!(samples[2], Rgb([255, 0, 0]));
        } else {
            panic!("Failed to get samples");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 800;
    let height = 600;
    let scale_factor = 4;
    
    let before_img = circle::draw_circle_no_aa(width, height);
    before_img.save("circle_no_aa.png")?;
    println!("Non-antialiased image saved as circle_no_aa.png");
    
    let fsaa_img = circle::draw_circle_fsaa(width, height, scale_factor);
    fsaa_img.save("circle_fsaa.png")?;
    println!("FSAA image saved as circle_fsaa.png");
    
    let comparison_fsaa = create_comparison_image(&before_img, &fsaa_img);
    comparison_fsaa.save("circle_comparison.png")?;
    println!("FSAA comparison image saved as circle_comparison_fsaa.png");
    
    Ok(())
}

fn create_comparison_image(before: &RgbImage, after: &RgbImage) -> RgbImage {
    let width = before.width();
    let height = before.height();
    
    let mut comparison = RgbImage::new(width * 2, height);
    
    for y in 0..height {
        for x in 0..width {
            comparison.put_pixel(x, y, *before.get_pixel(x, y));
        }
    }
    
    for y in 0..height {
        for x in 0..width {
            comparison.put_pixel(width + x, y, *after.get_pixel(x, y));
        }
    }
    
    for y in 0..height {
        comparison.put_pixel(width, y, Rgb([0, 0, 0]));
    }
    
    comparison
}
