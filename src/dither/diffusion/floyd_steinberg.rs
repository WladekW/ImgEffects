use image::{DynamicImage, ImageBuffer, Luma};

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
