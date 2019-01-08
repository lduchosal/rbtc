
use crate::encode::error::Error;
use crate::block::block::Block;
use crate::utils::hexdump;

#[test]
fn test() {

    let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
";


    let hex : Vec<u8> = hexdump::decode(dump);

    assert_eq!(hex.len(), 16);

    let result = Block::parse(&hex);
    assert!(result.is_err());

    let b = result.err().unwrap();
    assert_eq!(b, Error::InvalidLength);

}