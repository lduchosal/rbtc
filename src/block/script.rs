use crate::block::error::EncodeError;
use crate::block::varint;

use crate::primitives::script::Script;

use std::io::Read;
use std::io::Cursor;

pub(crate) fn parse_script(r: &mut Cursor<&Vec<u8>>) -> Result<Script, EncodeError> {

    let scriptlen = varint::parse_varint(r).map_err(|_| EncodeError::ScriptLen)?;
    let mut content = vec![0u8; scriptlen];
    let mut content_ref = content.as_mut_slice();
    r.read_exact(&mut content_ref).map_err(|_| EncodeError::ScriptContent)?;

    let result = Script {
        content: content
    };

    Ok(result)
}


#[cfg(test)]
mod test {

    use crate::block::script;
    use crate::block::error::EncodeError;

    use crate::primitives::script::Script;

    use std::io::Cursor;

    #[test]
    fn parse_script_0x00_then_1_byte() {

        let data : Vec<u8> = vec![0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::parse_script(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 1);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x00);
    }

    #[test]
    fn parse_script_0x01_then_1_byte() {

        let data : Vec<u8> = vec![0x01, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::parse_script(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 2);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x01);
    }

    #[test]
    fn parse_script_0x02_then_2_byte() {

        let data : Vec<u8> = vec![0x02, 0x00, 0x00];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::parse_script(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 3);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x02);
    }

    #[test]
    fn parse_script_0x10_then_10_byte() {

        let data : Vec<u8> = vec![0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::parse_script(&mut c);
        assert!(parsescript.is_ok());
        assert_eq!(c.position(), 0x11);

        let result : Script = parsescript.unwrap();
        assert_eq!(result.content.len(), 0x10);
    }


    #[test]
    fn parse_script_invalid_size_then_fail() {

        let data : Vec<u8> = vec![0x01 ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::parse_script(&mut c);
        assert!(parsescript.is_err());
        assert_eq!(c.position(), 0x01);


        if let Err(e) = parsescript {
            assert_eq!(e, EncodeError::ScriptContent);
        } else {
            panic!("should have failed");
        }

    }

    #[test]
    fn parse_script_invalid_content_then_fail() {

        let data : Vec<u8> = vec![ ];
        let mut c = Cursor::new(data.as_ref());
        let parsescript = script::parse_script(&mut c);
        assert!(parsescript.is_err());

        if let Err(e) = parsescript {
            assert_eq!(e, EncodeError::ScriptLen);
        } else {
            panic!("should have failed");
        }

    }
}