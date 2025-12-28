use rfd::FileDialog;
use std::io::{self, Write};
use std::path::PathBuf;

mod dither;

fn read(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    loop {
        println!(
            "\nTo exit: Ctrl + C\nAvailable formats: jpg, jpeg, png, webp, bmp, gif, ico, tiff"
        );

        // === Wybór wielu plików ===
        let inputs: Vec<PathBuf> = match FileDialog::new()
            .add_filter(
                "Images",
                &["jpg", "jpeg", "png", "webp", "bmp", "gif", "ico", "tiff"],
            )
            .set_title("Select input images")
            .pick_files()
        {
            Some(files) if !files.is_empty() => files,
            _ => {
                println!("No files selected. Exiting.");
                return;
            }
        };

        // === Wybór efektu ===
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

        // === Parametry ===
        let n = if matches!(effect.as_str(), "1" | "2" | "3") {
            Some(
                read("Matrix size (2 / 4 / 8 / 16 / 32 / 64):\n> ")
                    .parse::<usize>()
                    .unwrap(),
            )
        } else {
            None
        };

        let (low, high) = if matches!(effect.as_str(), "3" | "6") {
            let low: Vec<u8> = read("Low color (R G B):\n> ")
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();

            let high: Vec<u8> = read("High color (R G B):\n> ")
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();

            (
                Some([low[0], low[1], low[2]]),
                Some([high[0], high[1], high[2]]),
            )
        } else {
            (None, None)
        };

        // === Katalog wyjściowy ===
        let out_dir = FileDialog::new()
            .set_title("Select output directory")
            .pick_folder()
            .expect("No output directory selected");

        // === Przetwarzanie wielu plików ===
        for input in inputs {
            let img =
                image::open(&input).unwrap_or_else(|_| panic!("Cannot open image: {:?}", input));

            let stem = input.file_stem().unwrap().to_string_lossy();

            let out_path = match effect.as_str() {
                "1" => {
                    let out = dither::ordered::bayer::dither(n.unwrap(), &img);
                    let p = out_dir.join(format!("{stem}_bayer_gray.png"));
                    out.save(&p).unwrap();
                    p
                }
                "2" => {
                    let out = dither::ordered::bayer::dither_colored(n.unwrap(), &img);
                    let p = out_dir.join(format!("{stem}_bayer_color.png"));
                    out.save(&p).unwrap();
                    p
                }
                "3" => {
                    let out = dither::ordered::bayer::dither_duoton(
                        n.unwrap(),
                        &img,
                        low.unwrap(),
                        high.unwrap(),
                    );
                    let p = out_dir.join(format!("{stem}_bayer_duotone.png"));
                    out.save(&p).unwrap();
                    p
                }
                "4" => {
                    let out = dither::diffusion::floyd_steinberg::dither(&img);
                    let p = out_dir.join(format!("{stem}_fs_gray.png"));
                    out.save(&p).unwrap();
                    p
                }
                "5" => {
                    let out = dither::diffusion::floyd_steinberg::dither_colored(&img);
                    let p = out_dir.join(format!("{stem}_fs_color.png"));
                    out.save(&p).unwrap();
                    p
                }
                "6" => {
                    let out = dither::diffusion::floyd_steinberg::dither_duoton(
                        &img,
                        low.unwrap(),
                        high.unwrap(),
                    );
                    let p = out_dir.join(format!("{stem}_fs_duotone.png"));
                    out.save(&p).unwrap();
                    p
                }
                _ => unreachable!(),
            };

            println!("Saved: {:?}", out_path);
        }

        println!("\nAll files processed.\n");
    }
}
