mod pixels;
mod platform;
mod platformsdl2;

pub use pixels::{Pixels, BYTES_PER_PIXEL};
pub use platform::{Input, Platform};
pub use platformsdl2::PlatformSDL2;
