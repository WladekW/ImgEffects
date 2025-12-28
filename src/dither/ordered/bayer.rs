use super::bayer_matrices;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb};

fn dither_generic(x: u32, y: u32, luma: f32, matrix: &[u16], size: usize) -> f32 {
    let nx = (x as usize) % size;
    let ny = (y as usize) % size;
    let index = nx + ny * size;

    let limit = (matrix[index] as f32 + 1.0) / ((size * size + 1) as f32);
    if luma < limit { 0.0 } else { 1.0 }
}

fn dithered_val_calc(x: u32, y: u32, luma: f32, size: usize) -> f32 {
    let dithered_val = match size {
        2 => dither_generic(x, y, luma, &bayer_matrices::DITHER_MATRIX_2X2, 2),
        4 => dither_generic(x, y, luma, &bayer_matrices::DITHER_MATRIX_4X4, 4),
        8 => dither_generic(x, y, luma, &bayer_matrices::DITHER_MATRIX_8X8, 8),
        16 => dither_generic(x, y, luma, &bayer_matrices::DITHER_MATRIX_16X16, 16),
        32 => dither_generic(x, y, luma, &bayer_matrices::DITHER_MATRIX_32X32, 32),
        64 => dither_generic(x, y, luma, &bayer_matrices::DITHER_MATRIX_64X64, 64),
        _ => panic!("Unsupported dither size"),
    };
    dithered_val
}

pub fn dither(n: usize, img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (img_x, img_y) = img.dimensions();
    let mut img_out = ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let luma = 0.299 * r + 0.587 * g + 0.114 * b;

        let dithered_val = dithered_val_calc(x, y, luma, n);

        let luma_u8 = (dithered_val * 255.0) as u8;
        img_out.put_pixel(x, y, Luma([luma_u8]));
    }

    img_out
}

pub fn dither_colored(n: usize, img: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (img_x, img_y) = img.dimensions();
    let mut img_out = ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let luma = 0.299 * r + 0.587 * g + 0.114 * b;

        let dithered_val = dithered_val_calc(x, y, luma, n);

        let out_r = (r * dithered_val * 255.0) as u8;
        let out_g = (g * dithered_val * 255.0) as u8;
        let out_b = (b * dithered_val * 255.0) as u8;

        img_out.put_pixel(x, y, Rgb([out_r, out_g, out_b]));
    }

    img_out
}

pub fn dither_duoton(
    n: usize,
    img: &DynamicImage,
    lowcolor: [u8; 3],
    highcolor: [u8; 3],
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (img_x, img_y) = img.dimensions();
    let mut img_out = ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let luma = 0.299 * r + 0.587 * g + 0.114 * b;

        let dithered_val = dithered_val_calc(x, y, luma, n);

        let final_color = if dithered_val > 0.5 {
            highcolor
        } else {
            lowcolor
        };

        img_out.put_pixel(x, y, Rgb(final_color));
    }

    img_out
}
