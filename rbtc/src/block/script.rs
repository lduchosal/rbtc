use crate::encode::error::Error;
use crate::encode::encode::{Encodable, Decodable};

use std::io::{Read, Write, Cursor};

#[derive(Debug)]
pub struct Script {
    pub content: Vec<u8>
}

impl Decodable for Script {
    fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Script, Error> {

        trace!("decode");

        let content = <Vec<u8>>::decode(r).map_err(|_| Error::Script)?;
        let result = Script {
            content: content
        };

        Ok(result)
    }
}


#[cfg(test)]
mod test {

    use crate::encode::error::Error;
    use crate::encode::encode::{Encodable, Decodable};
    use crate::block::script::Script;

    use std::io::Cursor;

    #[test]
    fn decode_0x00_then_1_byte() {

        let data : Vec<u8> = vec![0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = Script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 1);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x00);
    }

    #[test]
    fn decode_0x01_then_1_byte() {

        let data : Vec<u8> = vec![0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = Script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 2);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x01);
    }

    #[test]
    fn decode_0x02_then_2_byte() {

        let data : Vec<u8> = vec![0x02, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = Script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 3);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x02);
    }

    #[test]
    fn decode_0x10_then_10_byte() {

        let data : Vec<u8> = vec![0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = Script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 0x11);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x10);
    }


    #[test]
    fn decode_invalid_size_then_fail() {

        let data : Vec<u8> = vec![0x01 ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = Script::decode(&mut c);
        assert!(parsescript.is_err());
        assert_eq!(c.position(), 0x01);


        if let Err(e) = parsescript {
            assert_eq!(e, Error::Script);
        } else {
            panic!("should have failed");
        }

    }

    #[test]
    fn decode_invalid_content_then_fail() {

        let data : Vec<u8> = vec![ ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = Script::decode(&mut c);
        assert!(parsescript.is_err());

        if let Err(e) = parsescript {
            assert_eq!(e, Error::Script);
        } else {
            panic!("should have failed");
        }

    }
}