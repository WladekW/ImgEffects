pub mod dither;

pub use dither::diffusion::floyd_steinberg::dither_colored as floyd_dither_colored;
pub use dither::diffusion::floyd_steinberg::dither_duoton as floyd_dither_duoton;
pub use dither::ordered::bayer::dither_colored as bayer_dither_colored;
pub use dither::ordered::bayer::dither_duoton as bayer_dither_duoton;
