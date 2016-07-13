//! The compact font format.

mod encoding;
mod font_set;
mod header;
mod number;
mod offset;

pub mod char_set;
pub mod index;
pub mod operation;

pub use self::char_set::CharSet;
pub use self::encoding::Encoding;
pub use self::font_set::FontSet;
pub use self::header::Header;
pub use self::number::Number;
pub use self::offset::{Offset, OffsetSize};
pub use self::operation::Operations;

/// A glyph identifier.
pub type GlyphID = u16;

/// A string identifier.
pub type StringID = u16;
