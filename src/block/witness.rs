use crate::block::error::ParseError;
use crate::block::varint;

use crate::primitives::witness::Witness;

use std::io::Read;
use std::io::Cursor;

pub(crate) fn parse_witnesses(r: &mut Cursor<&Vec<u8>>) -> Result<Vec<Witness>, ParseError> {

    let mut result : Vec<Witness> = Vec::new();
    let count = varint::parse_varint(r).map_err(|_| ParseError::WitnessesCount)?;
    for _ in 0..count {
        let witness = parse_witness(r)?;
        result.push(witness);
    }

    Ok(result)
}

pub(crate) fn parse_witness(r: &mut Cursor<&Vec<u8>>) -> Result<Witness, ParseError> {

    let varlen = varint::parse_varint(r).map_err(|_| ParseError::WitnessLen)?;
    let mut data = vec![0u8; varlen];
    let mut data_ref = data.as_mut_slice();
    r.read_exact(&mut data_ref).map_err(|_| ParseError::WitnessData)?;

    let result = Witness {
        data: data
    };

    Ok(result)
}



#[cfg(test)]
mod test {

    use crate::block::witness;
    use crate::block::error::ParseError;

    use crate::primitives::witness::Witness;

    use std::io::Cursor;

    #[test]
    fn parse_witness_0x00_then_1_byte() {

        let data : Vec<u8> = vec![0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsewitness = witness::parse_witness(&mut c);
        assert!(parsewitness.is_ok());
        assert_eq!(c.position(), 1);

        let result : Witness = parsewitness.unwrap();
        assert_eq!(result.data.len(), 0x00);
    }

    #[test]
    fn parse_witness_0x01_then_1_byte() {

        let data : Vec<u8> = vec![0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsewitness = witness::parse_witness(&mut c);
        assert!(parsewitness.is_ok());
        assert_eq!(c.position(), 2);

        let result : Witness = parsewitness.unwrap();
        assert_eq!(result.data.len(), 0x01);
    }

    #[test]
    fn parse_witness_0x02_then_2_byte() {

        let data : Vec<u8> = vec![0x02, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsewitness = witness::parse_witness(&mut c);
        assert!(parsewitness.is_ok());
        assert_eq!(c.position(), 3);

        let result : Witness = parsewitness.unwrap();
        assert_eq!(result.data.len(), 0x02);
    }

    #[test]
    fn parse_witness_0x10_then_10_byte() {

        let data : Vec<u8> = vec![0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
        let mut c = Cursor::new(data.as_ref());
        let parsewitness = witness::parse_witness(&mut c);
        assert!(parsewitness.is_ok());
        assert_eq!(c.position(), 0x11);

        let result : Witness = parsewitness.unwrap();
        assert_eq!(result.data.len(), 0x10);
    }


    #[test]
    fn parse_witness_invalid_size_then_fail() {

        let data : Vec<u8> = vec![0x01 ];
        let mut c = Cursor::new(data.as_ref());
        let parsewitness = witness::parse_witness(&mut c);
        assert!(parsewitness.is_err());
        assert_eq!(c.position(), 0x01);


        if let Err(e) = parsewitness {
            assert_eq!(e, ParseError::WitnessData);
        } else {
            panic!("should have failed");
        }

    }

    #[test]
    fn parse_witness_invalid_content_then_fail() {

        let data : Vec<u8> = vec![ ];
        let mut c = Cursor::new(data.as_ref());
        let parsewitness = witness::parse_witness(&mut c);
        assert!(parsewitness.is_err());

        if let Err(e) = parsewitness {
            assert_eq!(e, ParseError::WitnessLen);
        } else {
            panic!("should have failed");
        }

    }
}