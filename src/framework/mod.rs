mod pixels;
mod platform;
mod platformsdl2;

pub use pixels::{BYTES_PER_PIXEL, Pixels};
pub use platform::{Platform, Input};
pub use platformsdl2::PlatformSDL2;