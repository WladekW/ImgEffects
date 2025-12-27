use image::{GenericImageView, imageops::FilterType::Nearest};

mod dither;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open("C:/Users/Wlad/Downloads/miko.jpg").unwrap();
    let (img_x, img_y) = img.dimensions();

    println!("{}:{}", img_x, img_y);
    println!("{:?}", img.color());

    // let img_out = dither::ordered_dither::dither(8, &img);

    // img_out.save("C:/Users/Wlad/Downloads/gb.png")?;
    // println!("Img b/w saved!");

    // let img_out = dither::ordered_dither::dither_colored(8, &img);

    // img_out.save("C:/Users/Wlad/Downloads/clr.png")?;
    // println!("Img clr saved!");

    let img_out = dither::ordered::bayer::dither_colored(16, &img);

    img_out.save("C:/Users/Wlad/Downloads/duo.webp")?;
    println!("Img duo saved!");

    Ok(())
}
