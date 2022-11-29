extern crate postscript;

#[macro_use]
mod common;

macro_rules! operations(
    ($($operator:ident: [$($operand:expr),*],)*) => ({
        use postscript::compact1::{Operand, Operations, Operator};
        use std::collections::HashMap;
        let mut mapping = HashMap::new();
        let mut ordering = vec![];
        $(mapping.insert(Operator::$operator, vec![$($operand as Operand),*]);)*
        $(ordering.push(Operator::$operator);)*
        Operations { mapping, ordering }
    });
);

mod noto_sans {
    use postscript::Tape;

    use crate::common::{setup, setup_font_set, Fixture};

    #[test]
    fn char_strings() {
        let set = setup_font_set(Fixture::NotoSansJP);
        let tables = &set.char_strings;
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].len(), 17810);
    }

    #[test]
    fn header() {
        use postscript::compact1::Header;

        let mut tape = setup(Fixture::NotoSansJP);
        let table = ok!(tape.take::<Header>());
        assert_eq!(table.major, 1);
        assert_eq!(table.minor, 0);
        assert_eq!(table.header_size, 4);
        assert_eq!(table.offset_size, 4);
    }

    #[test]
    fn names() {
        use postscript::compact1::index::Names;
        use postscript::compact1::Header;

        let mut tape = setup(Fixture::NotoSansJP);
        let position = ok!(tape.position());
        let table = ok!(tape.take::<Header>());
        ok!(tape.jump(position + table.header_size as u64));
        let table = ok!(ok!(tape.take::<Names>()).into());
        assert_eq!(table.len(), 1);
        assert_eq!(&table[0], "NotoSansJP-Regular");
    }

    #[test]
    fn global_dictionaries() {
        use postscript::compact1::index::{Dictionaries, Names};
        use postscript::compact1::Header;

        let mut tape = setup(Fixture::NotoSansJP);
        let position = ok!(tape.position());
        let table = ok!(tape.take::<Header>());
        ok!(tape.jump(position + table.header_size as u64));
        let _ = ok!(ok!(tape.take::<Names>()).into());
        let table = ok!(ok!(tape.take::<Dictionaries>()).into());
        assert_eq!(table.len(), 1);
        let operations = operations!(
            ROS: [394, 395, 0],
            Notice: [391],
            FullName: [392],
            FamilyName: [393],
            Weight: [388],
            UnderlinePosition: [-150],
            FontBBox: [-1002, -1048, 2928, 1808],
            CIDFontVersion: [2.002],
            CIDCount: [65529],
            CharSet: [7068],
            CharStrings: [43287],
            FDSelect: [42687],
            FDArray: [43013],
        );
        assert_eq!(table[0].mapping, operations.mapping);
        assert_eq!(table[0].ordering, operations.ordering);
    }
}

mod source_serif {
    use crate::common::{setup_font_set, Fixture};

    #[test]
    fn char_sets() {
        use postscript::compact1::CharSet;

        let set = setup_font_set(Fixture::SourceSerifPro);
        let tables = &set.char_sets;
        assert_eq!(tables.len(), 1);
        match &tables[0] {
            &CharSet::Format1(..) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn char_strings() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let tables = &set.char_strings;
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].len(), 547);
    }

    #[test]
    fn encodings() {
        use postscript::compact1::Encoding;

        let set = setup_font_set(Fixture::SourceSerifPro);
        let encodings = &set.encodings;
        let strings = &set.strings;
        assert_eq!(encodings.len(), 1);
        match &encodings[0] {
            encoding @ &Encoding::Standard => {
                assert_eq!(ok!(strings.get(ok!(encoding.get(0)))), ".notdef");
                assert_eq!(ok!(strings.get(ok!(encoding.get(42)))), "asterisk");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn global_dictionaries() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.global_dictionaries;
        assert_eq!(table.len(), 1);
        let operations = operations!(
            Version: [709],
            Notice: [710],
            Copyright: [711],
            FullName: [712],
            FamilyName: [712],
            Weight: [388],
            FontBBox: [-178, -335, 1138, 918],
            CharSet: [8340],
            CharStrings: [8917],
            Private: [65, 33671],
        );
        assert_eq!(table[0].mapping, operations.mapping);
        assert_eq!(table[0].ordering, operations.ordering);
    }

    #[test]
    fn global_subroutines() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.global_subroutines;
        assert_eq!(table.len(), 181);
    }

    #[test]
    fn header() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.header;
        assert_eq!(table.major, 1);
        assert_eq!(table.minor, 0);
        assert_eq!(table.header_size, 4);
        assert_eq!(table.offset_size, 2);
    }

    #[test]
    fn local_dictionaries() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let tables = &set.local_dictionaries;
        assert_eq!(tables.len(), 1);
        let operations = operations!(
            BlueValues: [-20, 20, 473, 18, 34, 15, 104, 15, 10, 20, 40, 20],
            OtherBlues: [-249, 10],
            FamilyBlues: [-20, 20, 473, 18, 34, 15, 104, 15, 10, 20, 40, 20],
            FamilyOtherBlues: [-249, 10],
            BlueScale: [0.0375],
            BlueFuzz: [0],
            StdHW: [41],
            StdVW: [85],
            StemSnapH: [41, 15],
            StemSnapV: [85, 10],
            DefaultWidthX: [370],
            NominalWidthX: [604],
            Subrs: [65],
        );
        assert_eq!(tables[0].mapping, operations.mapping);
        assert_eq!(tables[0].ordering, operations.ordering);
    }

    #[test]
    fn local_subroutines() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let tables = &set.local_subroutines;
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].len(), 180);
    }

    #[test]
    fn names() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.names;
        assert_eq!(table.len(), 1);
        assert_eq!(&table[0], "SourceSerifPro-Regular");
    }

    #[test]
    fn strings() {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let table = &set.strings;
        assert_eq!(table.len(), 322);
        assert_eq!(ok!(table.get(0)), ".notdef");
        assert_eq!(ok!(table.get(175)), "Aring");
        assert_eq!(ok!(table.get(500)), "nine.tosf");
    }
}
