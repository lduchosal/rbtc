
use crate::block::block::Block;
use crate::encode::error::Error;
use crate::utils::hexdump;

#[test]
fn test() {

    let dump = "
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
00000000   00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00   ................
";


    let hex : Vec<u8> = hexdump::decode(dump);

    assert_eq!(hex.len(), 288);

    let result = Block::parse(&hex);
    assert!(result.is_err());

    let err = result.err().unwrap();
    assert_eq!(err, Error::RemainingContent)

}