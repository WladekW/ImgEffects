#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dither_core;

use eframe::egui;
use image::{DynamicImage, imageops};
use rfd::FileDialog;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("ImgEffects Pro"),
        ..Default::default()
    };

    eframe::run_native(
        "ImgEffects Pro",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum DitherAlgorythm {
    Original,
    Bayer,
    Floyd,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum DitherMode {
    Grayscale,
    Colored,
    Duoton,
}

struct MyApp {
    original_image: Option<DynamicImage>,
    raw_image: Option<DynamicImage>,
    texture: Option<egui::TextureHandle>,

    selected_algorythm: DitherAlgorythm,
    selected_mode: DitherMode,
    dither_bayer_size: usize,

    color_low: [u8; 3],
    color_high: [u8; 3],
    contrast: f32,

    zoom_factor: f32,
    target_width: u32,
    target_height: u32,
    lock_aspect_ratio: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            original_image: None,
            raw_image: None,
            texture: None,
            selected_algorythm: DitherAlgorythm::Original,
            selected_mode: DitherMode::Grayscale,
            dither_bayer_size: 2,
            color_low: [0, 0, 0],
            color_high: [255, 255, 255],
            contrast: 0.0,
            zoom_factor: 1.0,
            target_width: 0,
            target_height: 0,
            lock_aspect_ratio: true,
        }
    }
}

impl MyApp {
    fn apply_effect(&mut self) {
        let Some(mut img) = self.original_image.clone() else {
            return;
        };

        if self.target_width > 0 && self.target_height > 0 {
            img = img.resize_exact(
                self.target_width,
                self.target_height,
                imageops::FilterType::Nearest,
            );
        }

        if self.contrast != 0.0 {
            img = DynamicImage::ImageRgba8(imageops::contrast(&img, self.contrast));
        }

        if self.selected_mode == DitherMode::Grayscale {
            img = img.grayscale();
        }

        self.raw_image = Some(self.process_dithering(img));
        self.texture = None;
    }

    fn process_dithering(&self, img: DynamicImage) -> DynamicImage {
        match self.selected_algorythm {
            DitherAlgorythm::Original => img,
            DitherAlgorythm::Bayer => self.apply_bayer(img),
            DitherAlgorythm::Floyd => self.apply_floyd(img),
        }
    }

    fn apply_bayer(&self, img: DynamicImage) -> DynamicImage {
        let n = 2_usize.pow(self.dither_bayer_size as u32);
        match self.selected_mode {
            DitherMode::Grayscale | DitherMode::Colored => {
                dither_core::bayer_dither_colored(n, &img)
            }
            DitherMode::Duoton => {
                dither_core::bayer_dither_duoton(n, &img, self.color_low, self.color_high)
            }
        }
    }

    fn apply_floyd(&self, img: DynamicImage) -> DynamicImage {
        match self.selected_mode {
            DitherMode::Grayscale | DitherMode::Colored => dither_core::floyd_dither_colored(&img),
            DitherMode::Duoton => {
                dither_core::floyd_dither_duoton(&img, self.color_low, self.color_high)
            }
        }
    }

    fn load_image(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Images", &["jpg", "png", "webp", "bmp"])
            .pick_file()
        {
            if let Ok(img) = image::open(&path) {
                self.target_width = img.width();
                self.target_height = img.height();
                self.original_image = Some(img);
                self.apply_effect();
            }
        }
    }

    fn save_image(&self) {
        if let Some(img) = &self.raw_image {
            if let Some(path) = FileDialog::new().set_file_name("output.png").save_file() {
                let _ = img.save(path);
            }
        }
    }

    fn ui_file_section(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("📂 Load").clicked() {
                    self.load_image();
                }
                if ui.button("💾 Save").clicked() {
                    self.save_image();
                }
            });
        });
    }

    fn ui_resize_section(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.group(|ui| {
            ui.label("Resize");
            ui.horizontal(|ui| {
                let w = ui.add(egui::DragValue::new(&mut self.target_width).prefix("W: "));
                ui.checkbox(&mut self.lock_aspect_ratio, "Lock");
                let h = ui.add(egui::DragValue::new(&mut self.target_height).prefix("H: "));

                if w.changed() || h.changed() {
                    self.handle_aspect_ratio(w.changed(), h.changed());
                    changed = true;
                }
            });
            if ui.button("Reset Size").clicked() {
                if let Some(img) = &self.original_image {
                    self.target_width = img.width();
                    self.target_height = img.height();
                    changed = true;
                }
            }
        });
        changed
    }

    fn ui_algorithm_section(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.group(|ui| {
            ui.label("Algorithm");
            egui::ComboBox::from_id_salt("algo")
                .selected_text(format!("{:?}", self.selected_algorythm))
                .show_ui(ui, |ui| {
                    changed |= ui
                        .selectable_value(
                            &mut self.selected_algorythm,
                            DitherAlgorythm::Original,
                            "Original",
                        )
                        .changed();
                    changed |= ui
                        .selectable_value(
                            &mut self.selected_algorythm,
                            DitherAlgorythm::Bayer,
                            "Bayer",
                        )
                        .changed();
                    changed |= ui
                        .selectable_value(
                            &mut self.selected_algorythm,
                            DitherAlgorythm::Floyd,
                            "Floyd",
                        )
                        .changed();
                });

            if self.selected_algorythm == DitherAlgorythm::Bayer {
                let label = format!("Matrix: {}", 2_usize.pow(self.dither_bayer_size as u32));
                changed |= ui
                    .add(egui::Slider::new(&mut self.dither_bayer_size, 1..=6).text(label))
                    .changed();
            }
        });
        changed
    }

    fn ui_color_section(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.group(|ui| {
            ui.label("Color Mode");
            ui.horizontal(|ui| {
                changed |= ui
                    .selectable_value(&mut self.selected_mode, DitherMode::Grayscale, "Gray")
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.selected_mode, DitherMode::Colored, "RGB")
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.selected_mode, DitherMode::Duoton, "2-Bit")
                    .changed();
            });

            if self.selected_mode == DitherMode::Duoton {
                ui.separator();
                ui.horizontal(|ui| {
                    changed |= ui.color_edit_button_srgb(&mut self.color_low).changed();
                    if ui.button("<->").clicked() {
                        std::mem::swap(&mut self.color_low, &mut self.color_high);
                        changed = true;
                    }
                    changed |= ui.color_edit_button_srgb(&mut self.color_high).changed();
                });
            }
        });
        changed
    }

    fn handle_aspect_ratio(&mut self, width_changed: bool, height_changed: bool) {
        let Some(orig) = &self.original_image else {
            return;
        };
        if !self.lock_aspect_ratio {
            return;
        }

        let aspect = orig.width() as f32 / orig.height() as f32;
        if width_changed {
            self.target_height = (self.target_width as f32 / aspect).round() as u32;
        } else if height_changed {
            self.target_width = (self.target_height as f32 * aspect).round() as u32;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if i.modifiers.command && i.smooth_scroll_delta.y != 0.0 {
                let zoom_speed = 0.001;
                let delta = i.smooth_scroll_delta.y * zoom_speed;
                self.zoom_factor = (self.zoom_factor + delta).clamp(0.1, 10.0);
            }
        });

        egui::SidePanel::right("controls").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                self.ui_file_section(ui);

                ui.add_enabled_ui(self.original_image.is_some(), |ui| {
                    let mut needs_update = false;

                    needs_update |= self.ui_resize_section(ui);

                    ui.group(|ui| {
                        ui.label("Contrast");
                        needs_update |= ui
                            .add(egui::Slider::new(&mut self.contrast, -50.0..=50.0))
                            .changed();
                    });

                    needs_update |= self.ui_algorithm_section(ui);
                    needs_update |= self.ui_color_section(ui);

                    if needs_update {
                        self.apply_effect();
                    }

                    ui.separator();
                    ui.group(|ui| {
                        ui.label("Zoom");
                        ui.add(
                            egui::Slider::new(&mut self.zoom_factor, 0.1..=10.0)
                                .logarithmic(true)
                                .show_value(true),
                        );
                        if ui.button("Reset Zoom").clicked() {
                            self.zoom_factor = 1.0;
                        }
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_viewport(ui, ctx);
        });
    }
}

impl MyApp {
    fn render_viewport(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(raw_img) = &self.raw_image {
            let texture = self.texture.get_or_insert_with(|| {
                let pixels = raw_img.to_rgba8();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [raw_img.width() as _, raw_img.height() as _],
                    pixels.as_flat_samples().as_slice(),
                );
                ctx.load_texture("img", color_image, egui::TextureOptions::NEAREST)
            });

            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.image((texture.id(), texture.size_vec2() * self.zoom_factor));
                    });
                });
        } else {
            ui.centered_and_justified(|ui| {
                ui.heading("Load an image to start");
            });
        }
    }
}
