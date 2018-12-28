
use rbtc::business::error::ParseError;
use rbtc::business::block;
use rbtc::hexdump;

#[test]
fn test() {

    let dump = "
00000000   01 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
";


    let hex : Vec<u8> = hexdump::parse(dump);

    assert_eq!(hex.len(), 16);

    let result = block::parse(&hex);
    assert!(result.is_err());

    let b = result.err().unwrap();
    assert_eq!(b, ParseError::InvalidLength);

}