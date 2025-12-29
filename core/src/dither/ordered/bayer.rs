use super::bayer_matrices;
use image::{DynamicImage, RgbImage};
use rayon::prelude::*;

fn get_matrix_slice(n: usize) -> (&'static [u16], usize) {
    match n {
        2 => (&bayer_matrices::DITHER_MATRIX_2X2, 2),
        4 => (&bayer_matrices::DITHER_MATRIX_4X4, 4),
        8 => (&bayer_matrices::DITHER_MATRIX_8X8, 8),
        16 => (&bayer_matrices::DITHER_MATRIX_16X16, 16),
        32 => (&bayer_matrices::DITHER_MATRIX_32X32, 32),
        64 => (&bayer_matrices::DITHER_MATRIX_64X64, 64),
        _ => panic!("Unsupported dither size"),
    }
}

pub fn dither_colored(n: usize, img: &DynamicImage) -> DynamicImage {
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let mut buffer = rgb.into_raw();

    let (matrix, size) = get_matrix_slice(n);
    let mask = (size - 1) as usize;
    let matrix_len = size * size;

    buffer
        .par_chunks_exact_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = i % width as usize;
            let y = i / width as usize;

            let col = x & mask;
            let row = y & mask;
            let matrix_val = matrix[row * size + col];

            let threshold = ((matrix_val as u32 * 255) / matrix_len as u32) as u8;

            pixel[0] = if pixel[0] > threshold { 255 } else { 0 };
            pixel[1] = if pixel[1] > threshold { 255 } else { 0 };
            pixel[2] = if pixel[2] > threshold { 255 } else { 0 };
        });

    let img_out = RgbImage::from_raw(width, height, buffer).unwrap();
    DynamicImage::ImageRgb8(img_out)
}

pub fn dither_duoton(n: usize, img: &DynamicImage, low: [u8; 3], high: [u8; 3]) -> DynamicImage {
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let mut buffer = rgb.into_raw();

    let (matrix, size) = get_matrix_slice(n);
    let mask = (size - 1) as usize;
    let matrix_len = size * size;

    buffer
        .par_chunks_exact_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = i % width as usize;
            let y = i / width as usize;

            let luma =
                ((pixel[0] as u32 * 299) + (pixel[1] as u32 * 587) + (pixel[2] as u32 * 114))
                    / 1000;

            let col = x & mask;
            let row = y & mask;
            let matrix_val = matrix[row * size + col];
            let threshold = (matrix_val as u32 * 255) / matrix_len as u32; // u32 bo luma jest u32

            if luma > threshold {
                pixel.copy_from_slice(&high);
            } else {
                pixel.copy_from_slice(&low);
            }
        });

    let img_out = RgbImage::from_raw(width, height, buffer).unwrap();
    DynamicImage::ImageRgb8(img_out)
}
