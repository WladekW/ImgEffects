use std::io::{self, Write};

mod dither;

fn read(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    println!("\nAvailable formats: jpg, jpeg, png, webp, bmp, gif, ico, tiff");
    let input = read("Enter image path:\n> ");
    let img = image::open(&input).expect("Cannot open image");

    println!("\nChoose effect:");
    println!("1) Ordered dithering (grayscale)");
    println!("2) Ordered dithering (color)");
    println!("3) Ordered dithering (duotone)");
    println!("4) Floyd–Steinberg (grayscale)");
    println!("5) Floyd–Steinberg (color)");
    println!("6) Floyd–Steinberg (duotone)");

    let effect = loop {
        let choice = read("> ");
        match choice.as_str() {
            "1" | "2" | "3" | "4" | "5" | "6" => break choice,
            _ => println!("Choose 1–6"),
        }
    };

    let output = read("\nOutput path (default out.png):\n> ");
    let output = if output.is_empty() {
        "out.png".to_string()
    } else {
        output
    };

    match effect.as_str() {
        "1" => {
            let n: usize = read("Matrix size (2 / 4 / 8 / 16):\n> ").parse().unwrap();
            let out = dither::ordered::bayer::dither(n, &img);
            out.save(output).unwrap();
        }
        "2" => {
            let n: usize = read("Matrix size (2 / 4 / 8 / 16):\n> ").parse().unwrap();
            let out = dither::ordered::bayer::dither_colored(n, &img);
            out.save(output).unwrap();
        }
        "3" => {
            let n: usize = read("Matrix size (2 / 4 / 8 / 16):\n> ").parse().unwrap();
            let low = read("Low color (R G B):\n> ");
            let low: Vec<u8> = low.split_whitespace().map(|v| v.parse().unwrap()).collect();
            let high = read("High color (R G B):\n> ");
            let high: Vec<u8> = high
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            let out = dither::ordered::bayer::dither_duoton(
                n,
                &img,
                [low[0], low[1], low[2]],
                [high[0], high[1], high[2]],
            );
            out.save(output).unwrap();
        }
        "4" => {
            let out = dither::diffusion::floyd_steinberg::dither(&img);
            out.save(output).unwrap();
        }
        "5" => {
            let out = dither::diffusion::floyd_steinberg::dither_colored(&img);
            out.save(output).unwrap();
        }
        "6" => {
            let low = read("Low color (R G B):\n> ");
            let low: Vec<u8> = low.split_whitespace().map(|v| v.parse().unwrap()).collect();
            let high = read("High color (R G B):\n> ");
            let high: Vec<u8> = high
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            let out = dither::diffusion::floyd_steinberg::dither_duoton(
                &img,
                [low[0], low[1], low[2]],
                [high[0], high[1], high[2]],
            );
            out.save(output).unwrap();
        }
        _ => unreachable!(),
    }

    println!("\nDone.");
}
