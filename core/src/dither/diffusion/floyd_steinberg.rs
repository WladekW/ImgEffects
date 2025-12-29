use image::{DynamicImage, Rgb, RgbImage};

pub fn dither_colored(img: &DynamicImage) -> DynamicImage {
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();
    let w = w as usize;
    let h = h as usize;

    let mut buffer: Vec<i16> = rgb.as_raw().iter().map(|&b| b as i16).collect();

    for y in 0..(h - 1) {
        for x in 1..(w - 1) {
            let idx = (y * w + x) * 3;

            for c in 0..3 {
                let old_val = buffer[idx + c];

                let new_val = if old_val > 127 { 255 } else { 0 };
                let err = old_val - new_val;

                buffer[idx + c] = new_val;

                let idx_next = idx + 3 + c;
                buffer[idx_next] = buffer[idx_next].saturating_add((err * 7) >> 4);

                let idx_down_row = (y + 1) * w * 3 + x * 3;

                let idx_bl = idx_down_row - 3 + c;
                buffer[idx_bl] = buffer[idx_bl].saturating_add((err * 3) >> 4);

                let idx_b = idx_down_row + c;
                buffer[idx_b] = buffer[idx_b].saturating_add((err * 5) >> 4);

                let idx_br = idx_down_row + 3 + c;
                buffer[idx_br] = buffer[idx_br].saturating_add((err * 1) >> 4);
            }
        }
    }

    let raw_u8: Vec<u8> = buffer.into_iter().map(|v| v.clamp(0, 255) as u8).collect();

    let img_out = RgbImage::from_raw(w as u32, h as u32, raw_u8).unwrap();
    DynamicImage::ImageRgb8(img_out)
}

pub fn dither_duoton(img: &DynamicImage, low: [u8; 3], high: [u8; 3]) -> DynamicImage {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    let w = w as usize;
    let h = h as usize;

    let mut err_buffer: Vec<i16> = gray.as_raw().iter().map(|&x| x as i16).collect();

    let mut out_img = RgbImage::new(w as u32, h as u32);

    for y in 0..(h - 1) {
        for x in 1..(w - 1) {
            let idx = y * w + x;
            let old_val = err_buffer[idx];

            let (target_val_u8, color) = if old_val > 127 { (255, high) } else { (0, low) };

            out_img.put_pixel(x as u32, y as u32, Rgb(color));

            let err = old_val - target_val_u8;

            err_buffer[idx + 1] = err_buffer[idx + 1].saturating_add((err * 7) >> 4);

            let down_idx = (y + 1) * w + x;
            err_buffer[down_idx - 1] = err_buffer[down_idx - 1].saturating_add((err * 3) >> 4);
            err_buffer[down_idx] = err_buffer[down_idx].saturating_add((err * 5) >> 4);
            err_buffer[down_idx + 1] = err_buffer[down_idx + 1].saturating_add((err * 1) >> 4);
        }
    }

    DynamicImage::ImageRgb8(out_img)
}
