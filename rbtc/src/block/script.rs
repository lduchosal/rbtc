use crate::block::error::Error;
use crate::block::varint;

use crate::primitives::script::Script;

use std::io::{Read, Write, Cursor};


pub(crate) fn decode(r: &mut Cursor<&Vec<u8>>) -> Result<Script, Error> {

    let scriptlen = varint::decode(r).map_err(|_| Error::ScriptLen)?;
    let mut content = vec![0u8; scriptlen as usize];
    let mut content_ref = content.as_mut_slice();
    r.read_exact(&mut content_ref).map_err(|_| Error::ScriptContent)?;

    let result = Script {
        content: content
    };

    Ok(result)
}


#[cfg(test)]
mod test {

    use crate::block::script;
    use crate::block::error::Error;

    use crate::primitives::script::Script;

    use std::io::Cursor;

    #[test]
    fn decode_0x00_then_1_byte() {

        let data : Vec<u8> = vec![0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 1);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x00);
    }

    #[test]
    fn decode_0x01_then_1_byte() {

        let data : Vec<u8> = vec![0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 2);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x01);
    }

    #[test]
    fn decode_0x02_then_2_byte() {

        let data : Vec<u8> = vec![0x02, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 3);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x02);
    }

    #[test]
    fn decode_0x10_then_10_byte() {

        let data : Vec<u8> = vec![0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::decode(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 0x11);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x10);
    }


    #[test]
    fn decode_invalid_size_then_fail() {

        let data : Vec<u8> = vec![0x01 ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::decode(&mut c);
        assert!(parsescript.is_err());
        assert_eq!(c.position(), 0x01);


        if let Err(e) = parsescript {
            assert_eq!(e, Error::ScriptContent);
        } else {
            panic!("should have failed");
        }

    }

    #[test]
    fn decode_invalid_content_then_fail() {

        let data : Vec<u8> = vec![ ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::decode(&mut c);
        assert!(parsescript.is_err());

        if let Err(e) = parsescript {
            assert_eq!(e, Error::ScriptLen);
        } else {
            panic!("should have failed");
        }

    }
}