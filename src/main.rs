use image::{GenericImageView, imageops::FilterType::Nearest};
mod dither;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let img = image::open("C:/Users/Wlad/Downloads/flowers.jpg").unwrap().resize(520, 520, Nearest);

    let (img_x, img_y) = img.dimensions();

    println!("{}:{}", img_x, img_y);
    println!("{:?}", img.color());


    let img_out = dither::ordered_dither(4, &img);

    img_out.save("C:/Users/Wlad/Downloads/gb.png")?;
    println!("Obraz b/w zapisany!");

    let img_out = dither::ordered_dither_colored(2, &img);

    img_out.save("C:/Users/Wlad/Downloads/clr.png")?;
    println!("Obraz clr zapisany!");

    Ok(())
}
