//! The indices.

use {Result, Tape, Value, Walue};
use compact::{Offset, OffsetSize};

table! {
    @define
    #[doc(hidden)]
    pub Index {
        count       (u16         ), // count
        offset_size (OffsetSize  ), // offSize
        offsets     (Vec<Offset> ), // offset
        data        (Vec<Vec<u8>>), // data
    }
}

impl Value for Index {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let count = try!(tape.take::<u16>());
        if count == 0 {
            return Ok(Index::default());
        }
        let offset_size = try!(tape.take::<OffsetSize>());
        let mut offsets = Vec::with_capacity(count as usize + 1);
        for i in 0..(count as usize + 1) {
            let offset = try!(Offset::read(tape, offset_size));
            if i == 0 && offset != Offset(1) || i > 0 && offset <= offsets[i - 1] {
                raise!("found a malformed index");
            }
            offsets.push(offset);
        }
        let mut data = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            let size = (u32::from(offsets[i + 1]) - u32::from(offsets[i])) as usize;
            data.push(try!(Walue::read(tape, size)));
        }
        Ok(Index { count: count, offset_size: offset_size, offsets: offsets, data: data })
    }
}

deref! { Index::data => [Vec<u8>] }

macro_rules! index {
    ($(#[$attribute:meta])* pub $structure:ident) => (
        index! { @define $(#[$attribute])* pub $structure }
        index! { @implement $structure }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure {
            index: ::compact::index::Index,
        }
        deref! { $structure::index => ::compact::index::Index }
    );
    (@implement $structure:ident) => (
        impl ::tape::Value for $structure {
            #[inline]
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                Ok($structure { index: try!(::tape::Value::read(tape)) })
            }
        }
    );
}

mod char_strings;
mod dictionaries;
mod names;
mod strings;
mod subroutines;

pub use self::char_strings::CharStrings;
pub use self::dictionaries::Dictionaries;
pub use self::names::Names;
pub use self::strings::Strings;
pub use self::subroutines::Subroutines;
