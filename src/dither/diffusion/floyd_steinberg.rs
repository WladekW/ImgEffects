use image::{DynamicImage, ImageBuffer, Luma, Rgb};

// Oryginalna wersja grayscale
pub fn dither(img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    let mut buf: Vec<f32> = gray.pixels().map(|p| p[0] as f32 / 255.0).collect();
    let mut out_img = ImageBuffer::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let old = buf[idx];
            let new = if old < 0.5 { 0.0 } else { 1.0 };
            let err = old - new;
            out_img.put_pixel(x, y, Luma([(new * 255.0) as u8]));

            if x + 1 < w {
                buf[(y * w + (x + 1)) as usize] += err * 7.0 / 16.0;
            }
            if x > 0 && y + 1 < h {
                buf[((y + 1) * w + (x - 1)) as usize] += err * 3.0 / 16.0;
            }
            if y + 1 < h {
                buf[((y + 1) * w + x) as usize] += err * 5.0 / 16.0;
            }
            if x + 1 < w && y + 1 < h {
                buf[((y + 1) * w + (x + 1)) as usize] += err * 1.0 / 16.0;
            }
        }
    }
    out_img
}

// Wersja kolorowa - dithering dla każdego kanału RGB osobno
pub fn dither_colored(img: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();

    // Bufory dla każdego kanału
    let mut buf_r: Vec<f32> = Vec::with_capacity((w * h) as usize);
    let mut buf_g: Vec<f32> = Vec::with_capacity((w * h) as usize);
    let mut buf_b: Vec<f32> = Vec::with_capacity((w * h) as usize);

    for pixel in rgb.pixels() {
        buf_r.push(pixel[0] as f32 / 255.0);
        buf_g.push(pixel[1] as f32 / 255.0);
        buf_b.push(pixel[2] as f32 / 255.0);
    }

    let mut out_img = ImageBuffer::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;

            // Dithering dla R
            let old_r = buf_r[idx];
            let new_r = if old_r < 0.5 { 0.0 } else { 1.0 };
            let err_r = old_r - new_r;

            // Dithering dla G
            let old_g = buf_g[idx];
            let new_g = if old_g < 0.5 { 0.0 } else { 1.0 };
            let err_g = old_g - new_g;

            // Dithering dla B
            let old_b = buf_b[idx];
            let new_b = if old_b < 0.5 { 0.0 } else { 1.0 };
            let err_b = old_b - new_b;

            out_img.put_pixel(
                x,
                y,
                Rgb([
                    (new_r * 255.0) as u8,
                    (new_g * 255.0) as u8,
                    (new_b * 255.0) as u8,
                ]),
            );

            // Rozprzestrzenianie błędu dla wszystkich kanałów
            if x + 1 < w {
                let idx_right = (y * w + (x + 1)) as usize;
                buf_r[idx_right] += err_r * 7.0 / 16.0;
                buf_g[idx_right] += err_g * 7.0 / 16.0;
                buf_b[idx_right] += err_b * 7.0 / 16.0;
            }
            if x > 0 && y + 1 < h {
                let idx_bl = ((y + 1) * w + (x - 1)) as usize;
                buf_r[idx_bl] += err_r * 3.0 / 16.0;
                buf_g[idx_bl] += err_g * 3.0 / 16.0;
                buf_b[idx_bl] += err_b * 3.0 / 16.0;
            }
            if y + 1 < h {
                let idx_down = ((y + 1) * w + x) as usize;
                buf_r[idx_down] += err_r * 5.0 / 16.0;
                buf_g[idx_down] += err_g * 5.0 / 16.0;
                buf_b[idx_down] += err_b * 5.0 / 16.0;
            }
            if x + 1 < w && y + 1 < h {
                let idx_br = ((y + 1) * w + (x + 1)) as usize;
                buf_r[idx_br] += err_r * 1.0 / 16.0;
                buf_g[idx_br] += err_g * 1.0 / 16.0;
                buf_b[idx_br] += err_b * 1.0 / 16.0;
            }
        }
    }
    out_img
}

// Wersja duotone - dithering jasności, ale używa dwóch kolorów
pub fn dither_duoton(
    img: &DynamicImage,
    lowcolor: [u8; 3],
    highcolor: [u8; 3],
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    let mut buf: Vec<f32> = gray.pixels().map(|p| p[0] as f32 / 255.0).collect();
    let mut out_img = ImageBuffer::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let old = buf[idx];
            let new = if old < 0.5 { 0.0 } else { 1.0 };
            let err = old - new;

            // Wybór koloru na podstawie progowania
            let final_color = if new > 0.5 { highcolor } else { lowcolor };
            out_img.put_pixel(x, y, Rgb(final_color));

            if x + 1 < w {
                buf[(y * w + (x + 1)) as usize] += err * 7.0 / 16.0;
            }
            if x > 0 && y + 1 < h {
                buf[((y + 1) * w + (x - 1)) as usize] += err * 3.0 / 16.0;
            }
            if y + 1 < h {
                buf[((y + 1) * w + x) as usize] += err * 5.0 / 16.0;
            }
            if x + 1 < w && y + 1 < h {
                buf[((y + 1) * w + (x + 1)) as usize] += err * 1.0 / 16.0;
            }
        }
    }
    out_img
}
