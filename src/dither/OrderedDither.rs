use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb};


const DITHER_MATRIX_2X2: [u8; 4] = [
    0, 3,
    2, 1
];

const DITHER_MATRIX_4X4: [u8; 16] = [
    0, 8, 2, 10,
    12, 4 ,14 ,6,
    3 ,11 ,1 ,9,
    15 ,7 ,13 ,5
];


const DITHER_MATRIX_8X8: [u8; 64] = [
     0, 32,  8, 40,  2, 34, 10, 42,
    48, 16, 56, 24, 50, 18, 58, 26,
    12, 44,  4, 36, 14, 46,  6, 38,
    60, 28, 52, 20, 62, 30, 54, 22,
     3, 35, 11, 43, 1,  33,  9, 41,
    51, 19, 59, 27, 49, 17, 57, 25,
    15, 47,  7, 39, 13, 45,  5, 37,
    63, 31, 55, 23, 61, 29, 53, 21
];


fn dither_generic(
    x: u32,
    y: u32,
    luma: f32,
    matrix: &[u8],
    size: usize,
) -> f32 {
    let nx = (x as usize) % size;
    let ny = (y as usize) % size;
    let index = nx + ny * size;

    let limit = (matrix[index] as f32 + 1.0) / ((size * size + 1) as f32);
    if luma < limit { 0.0 } else { 1.0 }
}


pub fn ordered_dither(n: usize, img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (img_x, img_y) = img.dimensions();
    let mut img_out = ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let luma = 0.299 * r + 0.587 * g + 0.114 * b;

        let dithered_val = match n {
            2 => dither_generic(x, y, luma, &DITHER_MATRIX_2X2, 2),
            4 => dither_generic(x, y, luma, &DITHER_MATRIX_4X4, 4),
            8 => dither_generic(x, y, luma, &DITHER_MATRIX_8X8, 8),
            _ => panic!("Unsupported dither size"),
        };

        let luma_u8 = (dithered_val * 255.0) as u8;
        img_out.put_pixel(x, y, Luma([luma_u8]));
    }

    img_out
}

pub fn ordered_dither_colored(n: usize, img: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (img_x, img_y) = img.dimensions();
    let mut img_out = ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let luma = 0.299 * r + 0.587 * g + 0.114 * b;

        let dithered_val = match n {
            2 => dither_generic(x, y, luma, &DITHER_MATRIX_2X2, 2),
            4 => dither_generic(x, y, luma, &DITHER_MATRIX_4X4, 4),
            8 => dither_generic(x, y, luma, &DITHER_MATRIX_8X8, 8),
            _ => panic!("Unsupported dither size"),
        };

        let out_r = (r * dithered_val * 255.0) as u8;
        let out_g = (g * dithered_val * 255.0) as u8;
        let out_b = (b * dithered_val * 255.0) as u8;

        img_out.put_pixel(x, y, Rgb([out_r, out_g, out_b]));
    }

    img_out
}

pub fn ordered_dither_duoton(n: usize, img: &DynamicImage, lowcolor: [u8; 3], highcolor: [u8; 3] ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (img_x, img_y) = img.dimensions();
    let mut img_out = ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let luma = 0.299 * r + 0.587 * g + 0.114 * b;

        let dithered_val = match n {
            2 => dither_generic(x, y, luma, &DITHER_MATRIX_2X2, 2),
            4 => dither_generic(x, y, luma, &DITHER_MATRIX_4X4, 4),
            8 => dither_generic(x, y, luma, &DITHER_MATRIX_8X8, 8),
            _ => panic!("Unsupported dither size"),
        };

        let final_color = if dithered_val > 0.5{
            highcolor
        } else{
            lowcolor
        };

        img_out.put_pixel(x, y, Rgb(final_color));
    }

    img_out
}
