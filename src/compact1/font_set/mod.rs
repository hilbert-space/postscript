//! The font sets.

macro_rules! get(
    (@single $operations:expr, $operator:ident) => (
        match $operations.get_single(crate::compact1::Operator::$operator) {
            Some(crate::compact1::Number::Integer(value)) => value,
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("found no operation with operator ", stringify!($operator))),
        }
    );
    (@try @single $operations:expr, $operator:ident) => (
        match $operations.get_single(crate::compact1::Operator::$operator) {
            Some(crate::compact1::Number::Integer(value)) => Some(value),
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => None,
        }
    );
    (@double $operations:expr, $operator:ident) => (
        match $operations.get_double(crate::compact1::Operator::$operator) {
            Some((crate::compact1::Number::Integer(value0), crate::compact1::Number::Integer(value1))) => (value0, value1),
            Some(_) => raise!(concat!("found a malformed operation with operator ", stringify!($operator))),
            _ => raise!(concat!("found no operation with operator ", stringify!($operator))),
        }
    );
);

pub mod character_id_keyed;
pub mod character_name_keyed;

use crate::compact1::index::{CharStrings, Dictionaries, Names, Strings, Subroutines};
use crate::compact1::{CharSet, Encoding, Header, Operation, Operations, Operator};
use crate::{Result, Tape, Value, Walue};

/// A font set.
#[derive(Clone, Debug)]
pub struct FontSet {
    pub header: Header,
    pub names: Names,
    pub operations: Vec<Operations>,
    pub strings: Strings,
    pub subroutines: Subroutines,
    pub encodings: Vec<Encoding>,
    pub char_strings: Vec<CharStrings>,
    pub char_sets: Vec<CharSet>,
    pub records: Vec<Record>,
}

/// A record in a font set.
#[derive(Clone, Debug)]
pub enum Record {
    CharacterIDKeyed(character_id_keyed::Record),
    CharacterNameKeyed(character_name_keyed::Record),
}

impl Value for FontSet {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let header = tape.take::<Header>()?;
        tape.jump(position + header.header_size as u64)?;
        let names = tape.take::<Names>()?;
        let operations: Vec<_> = tape.take::<Dictionaries>()?.try_into()?;
        let strings = tape.take::<Strings>()?;
        let subroutines = tape.take::<Subroutines>()?;
        let mut encodings = vec![];
        let mut char_sets = vec![];
        let mut char_strings = vec![];
        let mut records = vec![];
        for (i, dictionary) in operations.iter().enumerate() {
            char_strings.push({
                tape.jump(position + get!(@single dictionary, CharStrings) as u64)?;
                tape.take_given::<CharStrings>(get!(@single dictionary, CharStringType))?
            });
            char_sets.push(match get!(@single dictionary, CharSet) {
                0 => CharSet::ISOAdobe,
                1 => CharSet::Expert,
                2 => CharSet::ExpertSubset,
                offset => {
                    tape.jump(position + offset as u64)?;
                    tape.take_given(char_strings[i].count as usize)?
                }
            });
            encodings.push(match get!(@single dictionary, Encoding) {
                0 => Encoding::Standard,
                1 => Encoding::Expert,
                offset => {
                    tape.jump(position + offset as u64)?;
                    tape.take()?
                }
            });
            records.push(tape.take_given((position, dictionary, &char_strings[i]))?);
        }
        Ok(Self {
            header,
            names,
            operations,
            strings,
            subroutines,
            encodings,
            char_strings,
            char_sets,
            records,
        })
    }
}

impl<'l> Walue<'l> for Record {
    type Parameter = (u64, &'l Operations, &'l CharStrings);

    fn read<T: Tape>(
        tape: &mut T,
        (position, dictionary, char_strings): Self::Parameter,
    ) -> Result<Self> {
        if let Some(Operation(Operator::ROS, _)) = <[_]>::get(dictionary, 0) {
            Ok(Record::CharacterIDKeyed(tape.take_given((
                position,
                dictionary,
                char_strings,
            ))?))
        } else {
            Ok(Record::CharacterNameKeyed(
                tape.take_given((position, dictionary))?,
            ))
        }
    }
}
