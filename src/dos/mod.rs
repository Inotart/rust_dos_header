mod decode;
mod encode;
mod to_print;

pub mod structs;
pub mod err;
pub use decode::decode;
pub use decode::decode_file;
pub use encode::encode;
pub use to_print::print;