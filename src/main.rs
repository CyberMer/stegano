use image::{GenericImageView, Rgba};
use ndarray::{Array, Array2};
use ndarray_linalg::c64;
use std::f32::consts::PI;

fn main() {
    let img = image::open("test.png").unwrap();
    let (width, height) = img.dimensions();

    let mut ycbcr_img: Vec<(u8, u8, u8)> = Vec::new();

    // Convert RGB to YCbCr color space
    for y in 0..height {
        for x in 0..width {
            let Rgba(rgb) = img.get_pixel(x, y);
            let ycbcr = rgb_to_ycbcr(rgb[0], rgb[1], rgb[2]);
            ycbcr_img.push(ycbcr);
        }
    }

    // Handle 8x8 blocks
    for y in (0..height).step_by(8) {
        for x in (0..width).step_by(8) {
            let mut block: Vec<(f32, f32, f32)> = Vec::new();

            for j in y..y + 8 {
                for i in x..x + 8 {
                    if let Some(pixel) = ycbcr_img.get((j * width + i) as usize) {
                        block.push((pixel.0 as f32, pixel.1 as f32, pixel.2 as f32));
                    } else {
                        block.push((0.0, 128.0, 128.0)); // Fill incomplete blocks with black
                    }
                }
            }

            // Convert block into 8x8 matrix
            let matrix = Array2::from_shape_vec((8, 8), block).unwrap();

            // Perform DCT
            let _dct_matrix = dct(&matrix);

            // Rest of the steps: Quantization, ZigZag ordering, and Huffman coding
            // ...
        }
    }
}

fn dct(block: &Array2<(f32, f32, f32)>) -> Array2<c64> {
    let mut result = Array::zeros((8, 8));
    let c8 = (1.0 / 2.0f32).sqrt();

    for u in 0..8 {
        for v in 0..8 {
            let mut sum = c64::new(0.0, 0.0);
            for x in 0..8 {
                for y in 0..8 {
                    let pixel = block[(x, y)];
                    let theta = PI * (x as f32 * u as f32 + y as f32 * v as f32) / 16.0;
                    sum += c64::new(pixel.0 as f64, pixel.1 as f64) * c64::from_polar(1.0f64, theta.into());
                }
            }
            let cu = if u == 0 { c8 } else { 1.0 };
            let cv = if v == 0 { c8 } else { 1.0 };
            result[(u, v)] = (cu as f64 * cv as f64 * sum * (1.0 / 4.0)).into();
        }
    }

    result
}





fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
    let cb = (128.0 - 0.168736 * r as f32 - 0.331264 * g as f32 + 0.5 * b as f32) as u8;
    let cr = (128.0 + 0.5 * r as f32 - 0.418688 * g as f32 - 0.081312 * b as f32) as u8;

    (y, cb, cr)
}
