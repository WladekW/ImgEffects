use image::{GenericImageView, imageops::FilterType::Nearest};

mod dither;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open("C:/Users/Wlad/Downloads/flowers.jpg").unwrap();
    // .resize(512, 512, Nearest);
    let (img_x, img_y) = img.dimensions();

    println!("{}:{}", img_x, img_y);
    println!("{:?}", img.color());

    // let img_out = dither::ordered_dither::dither(8, &img);

    // img_out.save("C:/Users/Wlad/Downloads/gb.png")?;
    // println!("Img b/w saved!");

    // let img_out = dither::ordered_dither::dither_colored(8, &img);

    // img_out.save("C:/Users/Wlad/Downloads/clr.png")?;
    // println!("Img clr saved!");

    let img_out = dither::diffusion::floyd_steinberg::dither(&img);
    img_out.save("C:/Users/Wlad/Downloads/miko2.png")?;
    println!("Saved !");
    Ok(())
}
